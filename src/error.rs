//! Error types for whisper-text operations.

use std::fmt;

/// Result type for whisper-text operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during encoding or decoding operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The cover text is too short to embed the secret message.
    CoverTextTooShort,

    /// No hidden message was found in the text.
    NoHiddenMessage,

    /// The hidden message is corrupted or invalid.
    CorruptedPayload,

    /// Invalid UTF-8 encountered during decoding.
    InvalidUtf8,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CoverTextTooShort => {
                write!(f, "cover text is too short to embed the secret message")
            }
            Error::NoHiddenMessage => {
                write!(f, "no hidden message found in the text")
            }
            Error::CorruptedPayload => {
                write!(f, "the hidden message is corrupted or invalid")
            }
            Error::InvalidUtf8 => {
                write!(f, "invalid UTF-8 encountered during decoding")
            }
        }
    }
}

impl std::error::Error for Error {}
