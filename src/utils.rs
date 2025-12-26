//! Utility functions for working with zero-width characters.

/// Checks if a character is a zero-width character used by this library.
///
/// # Arguments
///
/// * `c` - The character to check
///
/// # Returns
///
/// Returns `true` if the character is one of the zero-width characters
/// used in encoding/decoding.
pub fn is_zero_width_char(c: char) -> bool {
    matches!(c, '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}')
}

/// Strips all zero-width characters from the text, returning only visible content.
///
/// # Arguments
///
/// * `text` - The text to strip
///
/// # Returns
///
/// Returns a new String with all zero-width characters removed.
///
/// # Example
///
/// ```
/// use whisper_text::{encode, strip_hidden};
///
/// let encoded = encode("Hello", "secret").unwrap();
/// let visible = strip_hidden(&encoded);
/// assert_eq!(visible, "Hello");
/// ```
pub fn strip_hidden(text: &str) -> String {
    text.chars().filter(|&c| !is_zero_width_char(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_zero_width_char() {
        assert!(is_zero_width_char('\u{200B}'));
        assert!(is_zero_width_char('\u{200C}'));
        assert!(is_zero_width_char('\u{200D}'));
        assert!(is_zero_width_char('\u{FEFF}'));
        assert!(!is_zero_width_char('a'));
        assert!(!is_zero_width_char(' '));
    }

    #[test]
    fn test_strip_hidden() {
        let text = "H\u{200B}e\u{200C}l\u{200D}l\u{FEFF}o";
        assert_eq!(strip_hidden(text), "Hello");
    }

    #[test]
    fn test_strip_hidden_no_hidden() {
        let text = "Hello, World!";
        assert_eq!(strip_hidden(text), text);
    }
}
