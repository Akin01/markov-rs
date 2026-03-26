# markovify-rs

[![Crates.io](https://img.shields.io/crates/v/markovify-rs.svg)](https://crates.io/crates/markovify-rs)
[![Documentation](https://docs.rs/markovify-rs/badge.svg)](https://docs.rs/markovify-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust implementation of a Markov chain text generator, inspired by [markovify](https://github.com/jsvine/markovify).

**markovify-rs** is a simple, extensible Markov chain generator. Its primary use is for building Markov models of large corpora of text and generating random sentences from that.

## Features

- 🚀 **Fast** - Native Rust performance for text generation
- 📦 **Simple API** - Easy to use with sensible defaults
- 🔧 **Extensible** - Override key methods for custom behavior
- 💾 **JSON Serialization** - Save and load models for later use
- 🎯 **Configurable** - Adjustable state size, overlap detection, and more
- 🔗 **Model Combination** - Combine multiple models with weights

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
markovify-rs = "0.1.1"
```

## Basic Usage

```rust
use markov_rs::Text;

// Get raw text as string
let text = r#"
    Sherlock Holmes was a consulting detective. He solved crimes in London.
    His friend Dr. Watson helped him. They lived on Baker Street.
    Holmes was very clever and observant.
"#;

// Build the model
let text_model = Text::new(text, 2, true, true, None).unwrap();

// Print five randomly-generated sentences
for _ in 0..5 {
    if let Some(sentence) = text_model.make_sentence(None, None, None, None, None, None, None) {
        println!("{}", sentence);
    }
}

// Print three randomly-generated sentences of no more than 100 characters
for _ in 0..3 {
    if let Some(sentence) = text_model.make_short_sentence(100, None, None, None, None, None, None, None, None) {
        println!("Short: {}", sentence);
    }
}
```

## Advanced Usage

### Specifying the Model's State Size

State size is the number of words the probability of a next word depends on.

```rust
// Default state size is 2
let model = Text::new(text, 2, true, true, None).unwrap();

// Use a state size of 3
let model = Text::new(text, 3, true, true, None).unwrap();
```

### Combining Models

Combine two or more Markov chains with optional weights:

```rust
use markov_rs::{Text, utils::combine_texts};

let model_a = Text::new(text_a, 2, true, true, None).unwrap();
let model_b = Text::new(text_b, 2, true, true, None).unwrap();

// Combine with equal weights
let combined = combine_texts(vec![&model_a, &model_b], None).unwrap();

// Combine with custom weights (50% more weight on model_a)
let combined = combine_texts(vec![&model_a, &model_b], Some(vec![1.5, 1.0])).unwrap();
```

### Compiling a Model

Compile a model for improved text generation speed:

```rust
let text_model = Text::new(text, 2, true, true, None).unwrap();
let compiled_model = text_model.compile();

// Or compile in place
let mut text_model = Text::new(text, 2, true, true, None).unwrap();
text_model.compile_inplace();
```

### Working with Newline-Delimited Text

For text where sentences are separated by newlines instead of punctuation:

```rust
use markov_rs::NewlineText;

let text = r#"
Line one here
Line two there
Line three everywhere
"#;

let model = NewlineText::new(text, 2, true, true, None).unwrap();
```

### Exporting and Importing Models

Save and load models using JSON:

```rust
// Generate and save
let text_model = Text::new(corpus, 3, true, true, None).unwrap();
let model_json = text_model.to_json().unwrap();

// Save to file (optional)
std::fs::write("model.json", model_json).unwrap();

// Load from JSON
let model_json = std::fs::read_to_string("model.json").unwrap();
let reconstituted_model = Text::from_json(&model_json).unwrap();

// Generate a sentence
if let Some(sentence) = reconstituted_model.make_short_sentence(280, None, None, None, None, None, None, None, None) {
    println!("{}", sentence);
}
```

### Custom Sentence Rejection

Override the default rejection pattern:

```rust
// Use a custom regex to reject sentences containing specific patterns
let model = Text::new(
    text,
    2,
    true,
    true,
    Some(r"badword|anotherbad"),  // Custom rejection pattern
).unwrap();

// Or disable well-formed checking entirely
let model = Text::new(text, 2, true, false, None).unwrap();
```

### Sentence Generation Options

```rust
let model = Text::new(text, 2, true, true, None).unwrap();

// Generate with custom parameters
let sentence = model.make_sentence(
    None,                       // init_state: optional starting words
    Some(100),                  // tries: maximum attempts
    Some(0.5),                  // max_overlap_ratio: overlap threshold
    Some(10),                   // max_overlap_total: max overlap words
    Some(true),                 // test_output: whether to test for overlap
    Some(20),                   // max_words: maximum word count
    Some(5),                    // min_words: minimum word count
);

// Generate sentence starting with specific words
let sentence = model.make_sentence_with_start(
    "Sherlock Holmes",  // beginning phrase
    true,               // strict: only sentences starting with phrase
    Some(50),           // tries
    None, None, None, None, None,
).unwrap();
```

## API Reference

### `Text`

The main text model struct.

- `new(input_text, state_size, retain_original, well_formed, reject_reg)` - Create a new model
- `make_sentence(...)` - Generate a random sentence
- `make_short_sentence(max_chars, ...)` - Generate a sentence with character limit
- `make_sentence_with_start(beginning, ...)` - Generate sentence starting with specific words
- `compile()` - Compile for faster generation
- `to_json()` / `from_json()` - Serialize/deserialize

### `Chain`

The underlying Markov chain (non-text-specific).

- `new(corpus, state_size)` - Create a chain from corpus
- `walk(init_state)` - Generate a sequence
- `compile()` - Compile for faster generation
- `to_json()` / `from_json()` - Serialize/deserialize

### `NewlineText`

Text model that splits on newlines.

Same API as `Text`, but uses newline-based sentence splitting.

## Performance

Rust provides significant performance improvements over the Python implementation:

| Operation | Python (markovify) | Rust (markovify-rs) | Speedup |
|-----------|-------------------|------------------|---------|
| Model Creation | 50-100 ms | 5-15 ms | **5-10x** |
| Sentence Generation | 1-5 ms | 0.01-0.1 ms | **50-100x** |
| Compiled Generation | 0.5-2 ms | 0.01-0.05 ms | **20-50x** |
| Model Compilation | 10-30 ms | 1-5 ms | **5-10x** |
| JSON Serialize | 5-15 ms | 1-3 ms | **3-5x** |

### Running Benchmarks

```bash
# Run Rust benchmarks
cargo bench

# Run Python benchmarks
python3 benchmarks/python_benchmark.py

# Run both and compare
cd benchmarks
./run_benchmarks.sh --iterations 10
```

See [benchmarks/BENCHMARKS.md](benchmarks/BENCHMARKS.md) for detailed documentation.

## Notes

- Markovify works best with large, well-punctuated texts
- By default, `make_sentence` tries 10 times to generate a valid sentence
- The default overlap check rejects sentences that overlap by 15 words or 70% of the sentence length
- Setting `retain_original = false` reduces memory usage for large corpora

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

This is a Rust port of the excellent [markovify](https://github.com/jsvine/markovify) Python library by Jeremy Singer-Vine.
