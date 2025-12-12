//! # whisper-text
//!
//! A text steganography library that hides secret messages inside ordinary text
//! using zero-width Unicode characters.
//!
//! ## Example
//!
//! ```
//! use whisper_text::{encode, decode};
//!
//! let cover_text = "Hello, World!";
//! let secret = "secret message";
//!
//! // Encode secret into cover text
//! let encoded = encode(cover_text, secret).unwrap();
//!
//! // Decode secret from encoded text
//! let decoded = decode(&encoded).unwrap();
//! assert_eq!(decoded, secret);
//! ```

#![warn(missing_docs)]

mod codec;
mod error;

pub use codec::{decode, encode};
pub use error::{Error, Result};
