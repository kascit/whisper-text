# whisper-text

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight Rust library for text steganography that hides secret messages inside ordinary text using zero-width Unicode characters.

## Features

- 🔒 Hide messages in plain sight using zero-width Unicode characters
- 🎯 Lossless encoding and decoding
- 🌍 Full UTF-8 and Unicode support
- 🚀 Zero dependencies
- ✅ Comprehensive test coverage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
whisper-text = "0.1.0"
```

## Usage

```rust
use whisper_text::{encode, decode};

fn main() {
    let cover_text = "Hello, World!";
    let secret = "secret message";

    // Encode secret into cover text
    let encoded = encode(cover_text, secret).unwrap();
    
    // Decode secret from encoded text
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, secret);
}
```

## How It Works

The library converts secret messages to binary and encodes them using zero-width Unicode characters:
- `U+200B` (ZERO WIDTH SPACE) → binary '0'
- `U+200C` (ZERO WIDTH NON-JOINER) → binary '1'  
- `U+200D` (ZERO WIDTH JOINER) → start marker
- `U+FEFF` (ZERO WIDTH NO-BREAK SPACE) → end marker

These characters are invisible but preserved by computers.

## License

MIT
