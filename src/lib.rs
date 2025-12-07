//! # whisper-text
//!
//! Text steganography library using zero-width Unicode characters.

#![warn(missing_docs)]

mod codec;
mod error;

pub use codec::{decode, encode};
pub use error::{Error, Result};
