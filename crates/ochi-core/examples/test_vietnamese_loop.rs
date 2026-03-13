//! Vietnamese Text Test for Loop Detection

use ochi_core::ai::LoopDetector;

fn main() {
    println!("📝 Testing Loop Detection with Vietnamese Text\n");
    println!("={:=<60}\n", "");
    
    // Test 1: Normal Vietnamese text
    println!("Test 1: Normal Vietnamese text (no loop expected)\n");
    let mut detector = LoopDetector::new(10, 0.7);
    
    let vietnamese_text = "Xin chào các bạn. Hôm nay thời tiết thật đẹp. \
                          Tôi rất vui được gặp mọi người ở đây. \
                          Chúng ta hãy cùng nhau học tập và rèn luyện. \
                          Trí tuệ nhân tạo đang phát triển rất nhanh.";
    
    let tokens: Vec<&str> = vietnamese_text.split_whitespace().collect();
    println!("Input: {}\n", vietnamese_text);
    println!("Processing {} tokens...", tokens.len());
    
    let mut warnings = 0;
    for (i, token) in tokens.iter().enumerate() {
        let status = detector.check(token);
        if let Some(msg) = status.message() {
            println!("  [Token {}] {}", i, msg);
            warnings += 1;
        }
    }
    
    if warnings == 0 {
        println!("✅ No loops detected - Text is natural\n");
    } else {
        println!("⚠️  {} warnings found\n", warnings);
    }
    
    // Test 2: Repetitive Vietnamese text (should detect loop)
    println!("\nTest 2: Repetitive text (loop expected)\n");
    let mut detector = LoopDetector::new(10, 0.7);
    
    let repetitive_text = "tôi yêu coding tôi yêu coding tôi yêu coding";
    let tokens: Vec<&str> = repetitive_text.split_whitespace().collect();
    println!("Input: {}\n", repetitive_text);
    
    for (i, token) in tokens.iter().enumerate() {
        let status = detector.check(token);
        if status.is_loop() {
            println!("  [Token {}] ✅ LOOP DETECTED: {}", i, status.message().unwrap());
            break;
        }
    }
    
    // Test 3: Mixed English-Vietnamese technical text
    println!("\n\nTest 3: Mixed technical text\n");
    let mut detector = LoopDetector::new(10, 0.7);
    
    let tech_text = "Machine learning là một nhánh của AI. \
                    Deep learning sử dụng neural networks. \
                    Natural language processing giúp máy tính hiểu ngôn ngữ. \
                    Computer vision cho phép máy tính 'nhìn' hình ảnh.";
    
    let tokens: Vec<&str> = tech_text.split_whitespace().collect();
    println!("Input: {}\n", tech_text);
    
    for token in tokens {
        let status = detector.check(token);
        if let Some(msg) = status.message() {
            println!("  ⚠️  {}", msg);
        }
    }
    println!("✅ Technical text processed successfully\n");
    
    // Test 4: Poetry (may trigger false positives)
    println!("\nTest 4: Poetry (repetitive structure)\n");
    let mut detector = LoopDetector::new(10, 0.8);  // Higher threshold for poetry
    
    let poetry = "Đêm nay trăng sáng quá \
                  Đêm nay trăng sáng quá \
                  Lòng tôi buồn man mác \
                  Nhớ người phương xa xôi";
    
    let tokens: Vec<&str> = poetry.split_whitespace().collect();
    println!("Input: {}\n", poetry);
    
    for (i, token) in tokens.iter().enumerate() {
        let status = detector.check(token);
        if status.is_loop() {
            println!("  [Token {}] ⚠️  {}", i, status.message().unwrap());
        }
    }
    println!("✅ Poetry processed (with expected repetitions)\n");
    
    println!("\n={:=<60}", "");
    println!("✅ Vietnamese text tests completed!\n");
}
