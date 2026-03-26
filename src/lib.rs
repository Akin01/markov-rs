//! # markovify-rs
//!
//! A Rust implementation of a Markov chain text generator, inspired by [markovify](https://github.com/jsvine/markovify).
//!
//! Markovify-rs is a simple, extensible Markov chain generator. Its primary use is for building
//! Markov models of large corpora of text and generating random sentences from that.
//!
//! ## Basic Usage
//!
//! ```rust
//! use markov_rs::Text;
//!
//! // Build the model
//! let text = "Hello world. This is a test. The quick brown fox jumps.";
//! let model = Text::new(text, 2, true, true, None).unwrap();
//!
//! // Generate a random sentence
//! if let Some(sentence) = model.make_sentence(None, None, None, None, None, None, None) {
//!     println!("{}", sentence);
//! }
//!
//! // Generate a short sentence
//! if let Some(sentence) = model.make_short_sentence(100, None, None, None, None, None, None, None, None) {
//!     println!("Short: {}", sentence);
//! }
//! ```
//!
//! ## Features
//!
//! - Configurable state size
//! - Sentence generation with overlap detection
//! - Model compilation for faster generation
//! - Model combination with weights
//! - JSON export/import for persistence
//! - Newline-delimited text support

pub mod chain;
pub mod errors;
pub mod splitters;
pub mod text;
pub mod utils;

pub use chain::{Chain, BEGIN, END};
pub use errors::{MarkovError, Result};
pub use splitters::split_into_sentences;
pub use text::{NewlineText, Text};
pub use utils::{combine_chains, combine_models, combine_texts, CombinedResult, ModelRef};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
