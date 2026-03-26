# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2026-03-26

### Fixed
- Code formatting and type inference in tests and examples.

## [0.1.1] - 2026-03-26

### Changed
- Renamed crate from `markov-rs` to `markovify-rs` due to naming conflict on crates.io.

## [0.1.0] - 2026-03-26

### Added
- Initial Rust implementation of `markov-rs`.
- `Chain` struct for low-level Markov chain operations.
- `Text` struct for high-level text processing and generation.
- `NewlineText` for text where sentences are delimited by newlines.
- Support for state sizes greater than 1.
- Sentence generation with character and word limits.
- Sentence generation starting with specific words.
- Model compilation for improved generation performance.
- Model combination with weighted inputs.
- JSON serialization and deserialization for model persistence.
- Extensive unit and integration tests.
- Performance benchmarks comparing with Python `markovify`.
