//! Core encoding and decoding functionality for text steganography.

use crate::error::{Error, Result};

/// Zero-width Unicode characters used for binary encoding.
///
/// We use two zero-width characters to represent binary data:
/// - U+200B (ZERO WIDTH SPACE) represents binary '0'
/// - U+200C (ZERO WIDTH NON-JOINER) represents binary '1'
const ZERO_BIT: char = '\u{200B}'; // ZERO WIDTH SPACE
const ONE_BIT: char = '\u{200C}'; // ZERO WIDTH NON-JOINER

/// Marker to indicate the start of the hidden message.
const START_MARKER: &str = "\u{200D}"; // ZERO WIDTH JOINER

/// Marker to indicate the end of the hidden message.
const END_MARKER: &str = "\u{FEFF}"; // ZERO WIDTH NO-BREAK SPACE

/// Encodes a secret message into cover text using zero-width Unicode characters.
///
/// The secret message is converted to binary and embedded using zero-width
/// characters between the characters of the cover text. The encoded text
/// appears unchanged to readers but contains the hidden message.
///
/// # Arguments
///
/// * `cover_text` - The visible text that will contain the hidden message
/// * `secret` - The secret message to hide
///
/// # Returns
///
/// Returns the cover text with the secret message embedded using zero-width
/// characters, or an error if the cover text is too short.
///
/// # Example
///
/// ```
/// use whisper_text::encode;
///
/// let encoded = encode("Hello, World!", "secret").unwrap();
/// // Encoded text contains all original characters plus hidden zero-width chars
/// assert!(encoded.len() > "Hello, World!".len());
/// ```
pub fn encode(cover_text: &str, secret: &str) -> Result<String> {
    if cover_text.is_empty() {
        return Err(Error::CoverTextTooShort);
    }

    let secret_bytes = secret.as_bytes();

    // Convert secret to binary representation using zero-width chars
    let mut hidden = String::from(START_MARKER);

    for &byte in secret_bytes {
        for bit_pos in (0..8).rev() {
            let bit = (byte >> bit_pos) & 1;
            hidden.push(if bit == 1 { ONE_BIT } else { ZERO_BIT });
        }
    }

    hidden.push_str(END_MARKER);

    // Insert hidden message after the first character of cover text
    let mut chars = cover_text.chars();
    let mut result = String::new();

    if let Some(first_char) = chars.next() {
        result.push(first_char);
        result.push_str(&hidden);
        result.extend(chars);
    }

    Ok(result)
}

/// Decodes a hidden message from text containing zero-width Unicode characters.
///
/// Extracts and decodes the secret message that was embedded using the `encode`
/// function. The function looks for the start and end markers and decodes the
/// binary data between them.
///
/// # Arguments
///
/// * `encoded_text` - Text containing a hidden message
///
/// # Returns
///
/// Returns the decoded secret message, or an error if no valid message is found
/// or if the message is corrupted.
///
/// # Example
///
/// ```
/// use whisper_text::{encode, decode};
///
/// let encoded = encode("Hello, World!", "secret").unwrap();
/// let decoded = decode(&encoded).unwrap();
/// assert_eq!(decoded, "secret");
/// ```
pub fn decode(encoded_text: &str) -> Result<String> {
    // Find start and end markers
    let start_pos = encoded_text.find(START_MARKER);
    let end_pos = encoded_text.find(END_MARKER);

    match (start_pos, end_pos) {
        (Some(start), Some(end)) if start < end => {
            // Extract the hidden section (between markers)
            let hidden_start = start + START_MARKER.len();
            let hidden_section = &encoded_text[hidden_start..end];

            // Decode the binary data
            let mut bytes = Vec::new();
            let mut current_byte = 0u8;
            let mut bit_count = 0;

            for ch in hidden_section.chars() {
                let bit = match ch {
                    ZERO_BIT => 0,
                    ONE_BIT => 1,
                    _ => continue, // Ignore non-encoding characters
                };

                current_byte = (current_byte << 1) | bit;
                bit_count += 1;

                if bit_count == 8 {
                    bytes.push(current_byte);
                    current_byte = 0;
                    bit_count = 0;
                }
            }

            // Check if we have incomplete bits (corruption)
            if bit_count != 0 {
                return Err(Error::CorruptedPayload);
            }

            // Convert bytes to string
            String::from_utf8(bytes).map_err(|_| Error::InvalidUtf8)
        }
        (Some(_), Some(_)) => Err(Error::CorruptedPayload),
        _ => Err(Error::NoHiddenMessage),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic() {
        let result = encode("Hello", "hi");
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_empty_cover() {
        let result = encode("", "secret");
        assert_eq!(result, Err(Error::CoverTextTooShort));
    }

    #[test]
    fn test_round_trip_simple() {
        let cover = "Hello, World!";
        let secret = "secret";

        let encoded = encode(cover, secret).unwrap();
        let decoded = decode(&encoded).unwrap();

        assert_eq!(decoded, secret);
    }

    #[test]
    fn test_round_trip_unicode() {
        let cover = "Hello, 世界! 👋";
        let secret = "Unicode: 你好 🚀";

        let encoded = encode(cover, secret).unwrap();
        let decoded = decode(&encoded).unwrap();

        assert_eq!(decoded, secret);
    }

    #[test]
    fn test_decode_no_hidden_message() {
        let plain_text = "This is just plain text";
        let result = decode(plain_text);

        assert_eq!(result, Err(Error::NoHiddenMessage));
    }

    #[test]
    fn test_round_trip_empty_secret() {
        let cover = "Cover";
        let secret = "";

        let encoded = encode(cover, secret).unwrap();
        let decoded = decode(&encoded).unwrap();

        assert_eq!(decoded, secret);
    }

    #[test]
    fn test_round_trip_long_message() {
        let cover = "The quick brown fox jumps over the lazy dog.";
        let secret = "This is a much longer secret message with multiple words!";

        let encoded = encode(cover, secret).unwrap();
        let decoded = decode(&encoded).unwrap();

        assert_eq!(decoded, secret);
    }

    #[test]
    fn test_round_trip_special_chars() {
        let cover = "Cover text";
        let secret = "!@#$%^&*()_+-=[]{}|;':\",./<>?";

        let encoded = encode(cover, secret).unwrap();
        let decoded = decode(&encoded).unwrap();

        assert_eq!(decoded, secret);
    }

    #[test]
    fn test_deterministic_encoding() {
        let cover = "Hello";
        let secret = "test";

        let encoded1 = encode(cover, secret).unwrap();
        let encoded2 = encode(cover, secret).unwrap();

        assert_eq!(encoded1, encoded2);
    }

    #[test]
    fn test_encoded_text_preserves_visible_content() {
        let cover = "Hello, World!";
        let secret = "secret";

        let encoded = encode(cover, secret).unwrap();

        // The visible text should still contain all original characters
        let visible: String = encoded
            .chars()
            .filter(|&c| c != ZERO_BIT && c != ONE_BIT && c != '\u{200D}' && c != '\u{FEFF}')
            .collect();

        assert_eq!(visible, cover);
    }

    #[test]
    fn test_multiple_messages() {
        let cover = "Test";
        let secrets = vec!["a", "ab", "abc", "test123", "🔒"];

        for secret in secrets {
            let encoded = encode(cover, secret).unwrap();
            let decoded = decode(&encoded).unwrap();
            assert_eq!(decoded, secret, "Failed for secret: {}", secret);
        }
    }
}
