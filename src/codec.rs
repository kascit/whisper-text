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
