//! Knowledge Base Loader - Load docs/skills into Thổ DB Memory
//!
//! Usage: cargo run --bin ochi-load-kb

use ochi_ngu_hanh::{ThoAgent, MockEmbedding, EmbeddingProvider};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("🦀 Ochi Knowledge Base Loader\n");

    // Initialize Thổ DB
    let db_path = PathBuf::from("data/tho.db");

    // Ensure data directory exists
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create data directory");
        }
    }

    let tho = match ThoAgent::new(db_path.clone()) {
        Ok(agent) => {
            println!("✅ Connected to Thổ DB: {:?}", db_path);
            agent
        }
        Err(e) => {
            eprintln!("❌ Failed to open Thổ DB: {}", e);
            return;
        }
    };
    
    // Initialize embedding provider (Mock for now - deterministic hash-based)
    let embedder = MockEmbedding::new(384);
    println!("✅ Using Mock Embedding (384 dimensions)\n");
    
    // Load ALL knowledge from refs subdirectories
    let refs_dir = PathBuf::from("refs");
    if refs_dir.exists() {
        println!("\n📚 Loading ALL knowledge from refs/...");
        load_all_refs(&tho, &embedder, &refs_dir).expect("Failed to load refs");
    } else {
        println!("⚠️  refs directory not found");
    }

    // Load knowledge from skills
    let skills_dir = PathBuf::from("skills");
    if skills_dir.exists() {
        println!("\n📚 Loading knowledge from skills...");
        load_directory(&tho, &embedder, &skills_dir, "skills").expect("Failed to load skills");
    } else {
        println!("⚠️  skills directory not found");
    }

    // Load knowledge from docs
    let docs_dir = PathBuf::from("docs");
    if docs_dir.exists() {
        println!("\n📚 Loading knowledge from docs...");
        load_directory(&tho, &embedder, &docs_dir, "ochi-docs").expect("Failed to load docs");
    } else {
        println!("⚠️  docs directory not found");
    }
    
    // Show stats
    match tho.get_memory_stats() {
        Ok(stats) => {
            println!("\n📊 Memory Stats:");
            println!("   Chunks: {}", stats.chunk_count);
            println!("   Embeddings: {}", stats.embedding_count);
            println!("   Cache: {}", stats.cache_size);
        }
        Err(e) => eprintln!("❌ Failed to get stats: {}", e),
    }
    
    println!("\n✅ Knowledge Base loaded successfully!");
}

fn load_all_refs<E: EmbeddingProvider>(tho: &ThoAgent, embedder: &E, refs_dir: &Path) -> std::io::Result<()> {
    let mut total_count = 0;
    let mut total_bytes = 0;

    // Load each subdirectory in refs
    for entry in fs::read_dir(refs_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let folder_name = path.file_name().unwrap_or_default().to_string_lossy();
            println!("   📂 Loading {}...", folder_name);
            load_directory_recursive(tho, embedder, &path, &format!("refs/{}", folder_name), &mut total_count, &mut total_bytes)?;
        }
    }

    println!("   ✅ Total loaded: {} files ({} KB)", total_count, total_bytes / 1024);
    Ok(())
}

fn load_directory<E: EmbeddingProvider>(tho: &ThoAgent, embedder: &E, dir: &Path, source: &str) -> std::io::Result<()> {
    let mut count = 0;
    let mut total_bytes = 0;

    load_directory_recursive(tho, embedder, dir, source, &mut count, &mut total_bytes)?;

    println!("   ✅ Loaded {} files ({} KB)", count, total_bytes / 1024);
    Ok(())
}

fn load_directory_recursive<E: EmbeddingProvider>(
    tho: &ThoAgent,
    embedder: &E,
    dir: &Path,
    source: &str,
    count: &mut usize,
    total_bytes: &mut usize,
) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Skip git and target directories
            if path.file_name().map_or(false, |n| n == ".git" || n == "target" || n == "node_modules") {
                continue;
            }
            load_directory_recursive(tho, embedder, &path, source, count, total_bytes)?;
        } else if path.extension().map_or(false, |ext| ext == "md") {
            // Load markdown file
            if let Ok(content) = fs::read_to_string(&path) {
                let relative_path = path.strip_prefix("refs")
                    .or_else(|_| path.strip_prefix("skills"))
                    .or_else(|_| path.strip_prefix("docs"))
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();
                
                // Store chunk
                match tho.store_chunk(&content, Some(&relative_path), Some(source)) {
                    Ok(chunk_id) => {
                        *count += 1;
                        *total_bytes += content.len();
                        
                        // Generate REAL embedding (Mock - hash-based)
                        match embedder.embed(&content) {
                            Ok(embedding) => {
                                let _ = tho.store_embedding(chunk_id, "mock", &embedding);
                            }
                            Err(e) => {
                                eprintln!("   ⚠️  Embedding failed: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("   ⚠️  Failed to store {:?}: {}", path, e);
                    }
                }
            }
        }
    }
    
    Ok(())
}
