use whisper_text::{decode, encode, strip_hidden};

fn main() {
    println!("=== Whisper Text Demo ===\n");

    let cover = "The quick brown fox jumps over the lazy dog.";
    let secret = "This is a secret message! 🔒";

    println!("Cover text: {}", cover);
    println!("Secret: {}\n", secret);

    // Encode
    let encoded = encode(cover, secret).expect("Failed to encode");
    println!("Encoded text (looks same): {}", encoded);
    println!("Encoded length: {} bytes\n", encoded.len());

    // Decode
    let decoded = decode(&encoded).expect("Failed to decode");
    println!("Decoded secret: {}", decoded);

    // Strip hidden
    let visible = strip_hidden(&encoded);
    println!("\nVisible text after stripping: {}", visible);

    // Verify
    assert_eq!(decoded, secret);
    assert_eq!(visible, cover);
    println!("\n✓ All assertions passed!");
}
