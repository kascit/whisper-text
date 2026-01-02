use std::fs;
use whisper_text::{decode, encode};

fn main() {
    let cover = "Hello, World! This is a test.";
    let secret = "Hidden message 🔐";

    println!("=== File Test Demo ===\n");

    // Encode
    let encoded = encode(cover, secret).expect("Failed to encode");

    // Write to file
    fs::write("test_output.txt", &encoded).expect("Failed to write file");
    println!("✓ Written encoded text to test_output.txt");
    println!("  Open the file - it will look like: {}", cover);

    // Read from file
    let read_back = fs::read_to_string("test_output.txt").expect("Failed to read file");
    
    // Decode
    let decoded = decode(&read_back).expect("Failed to decode");
    
    println!("\n✓ Read back and decoded: {}", decoded);
    assert_eq!(decoded, secret);
    println!("\n✓ Round-trip through file successful!");
    println!("\nNote: Open test_output.txt in a text editor - the hidden message is invisible!");
}
