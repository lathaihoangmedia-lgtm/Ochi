//! Test NLP với Knowledge Base đã load
//!
//! Usage: cargo run --bin ochi-test-nlp -- "query here"

use ochi_ngu_hanh::ThoAgent;
use ochi_trung_cung::HybridNLPProcessor;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("🦀 Ochi NLP Test với Knowledge Base\n");
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ochi-test-nlp <query>");
        eprintln!("Example: ochi-test-nlp \"how to create a skill\"");
        return;
    }
    
    let query = args[1..].join(" ");
    
    // Initialize Thổ DB
    let db_path = PathBuf::from("data/tho.db");
    let tho = match ThoAgent::new(db_path.clone()) {
        Ok(agent) => agent,
        Err(e) => {
            eprintln!("❌ Failed to open Thổ DB: {}", e);
            return;
        }
    };
    
    // Initialize Hybrid NLP với Thổ DB
    let nlp = match HybridNLPProcessor::with_tho_agent(db_path) {
        Ok(nlp) => nlp,
        Err(e) => {
            eprintln!("❌ Failed to initialize NLP: {}", e);
            return;
        }
    };
    
    println!("📚 Knowledge Base Stats:");
    match tho.get_memory_stats() {
        Ok(stats) => {
            println!("   Chunks: {}", stats.chunk_count);
            println!("   Embeddings: {}", stats.embedding_count);
            println!("   Cache: {}", stats.cache_size);
        }
        Err(e) => eprintln!("❌ Failed to get stats: {}", e),
    }
    
    println!("\n🔍 Query: \"{}\"\n", query);
    
    // Test 1: Hybrid NLP Process
    println!("1️⃣ Hybrid NLP Processing:");
    let intent = nlp.process(&query);
    println!("   Operation: {}", intent.operation);
    println!("   Layer: {}", intent.layer);
    println!("   Confidence: {:.1}%", intent.confidence * 100.0);
    if let Some(context) = &intent.context {
        println!("   Context: {}", context);
    }
    
    // Test 2: Memory Search (FTS5)
    println!("\n2️⃣ Memory Search (FTS5 + Hybrid):");
    let results = nlp.memory_search(&query, 5);
    
    if results.is_empty() {
        println!("   No results found");
    } else {
        for (i, result) in results.iter().enumerate() {
            println!("   {}. {} (score: {:.4})", i + 1, result.source, result.combined_score);
            // Show first 100 chars of content
            let preview = result.content.chars().take(100).collect::<String>();
            println!("      Preview: {}...", preview.trim());
        }
    }
    
    // Test 3: Operation Dictionary
    println!("\n3️⃣ Operation Dictionary:");
    if let Some(op) = nlp.get_operation(&intent.operation) {
        println!("   Name: {}", op.name);
        println!("   Description: {}", op.description);
        println!("   Aliases: {}", op.aliases.join(", "));
    } else {
        println!("   No matching operation found");
    }
    
    println!("\n✅ Test complete!");
}
