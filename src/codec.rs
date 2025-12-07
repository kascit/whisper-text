//! Core encoding and decoding functionality for text steganography.

use crate::error::{Error, Result};

/// Zero-width Unicode characters used for binary encoding.
/// 
/// We use two zero-width characters to represent binary data:
/// - U+200B (ZERO WIDTH SPACE) represents binary '0'
/// - U+200C (ZERO WIDTH NON-JOINER) represents binary '1'
const ZERO_BIT: char = '\u{200B}'; // ZERO WIDTH SPACE
const ONE_BIT: char = '\u{200C}';  // ZERO WIDTH NON-JOINER

/// Marker to indicate the start of the hidden message.
const START_MARKER: &str = "\u{200D}"; // ZERO WIDTH JOINER

/// Marker to indicate the end of the hidden message.
const END_MARKER: &str = "\u{FEFF}"; // ZERO WIDTH NO-BREAK SPACE

/// Encodes a secret message into cover text using zero-width Unicode characters.
///
/// The secret message is converted to binary and embedded using zero-width
/// characters between the characters of the cover text.
///
/// # Arguments
///
/// * `cover_text` - The visible text that will contain the hidden message
/// * `secret` - The secret message to hide
///
/// # Returns
///
/// Returns the cover text with the secret message embedded, or an error if
/// the cover text is too short.
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
