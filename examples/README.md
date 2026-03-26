# Markov-RS Examples

This directory contains example programs demonstrating various use cases of the markov-rs library.

## Running Examples

Run any example with:

```bash
cargo run --example <example_name>
```

## Available Examples

### 1. Basic Usage (`basic.rs`)
Demonstrates fundamental usage:
- Creating a text model
- Generating random sentences
- Generating short sentences
- Using generation constraints

```bash
cargo run --example basic
```

### 2. Sentence with Start (`sentence_with_start.rs`)
Shows how to generate sentences beginning with specific words:
- Strict mode (must start with phrase)
- Non-strict mode (contains phrase)
- Error handling

```bash
cargo run --example sentence_with_start
```

### 3. Serialization (`serialization.rs`)
Demonstrates saving and loading models:
- JSON serialization
- File I/O
- Model persistence
- Verifying loaded models

```bash
cargo run --example serialization
```

### 4. Model Combination (`combine.rs`)
Shows how to merge multiple models:
- Equal weight combination
- Weighted combination
- Different writing styles
- Hybrid models

```bash
cargo run --example combine
```

### 5. Compiled Performance (`compiled.rs`)
Demonstrates performance benefits of compilation:
- Timing comparisons
- Speedup measurements
- When to compile

```bash
cargo run --example compiled
```

### 6. Newline Text (`newline_text.rs`)
Shows usage with line-delimited text:
- Poetry generation
- Chat log simulation
- Line-based corpora

```bash
cargo run --example newline_text
```

### 7. File Processing (`file_processing.rs`)
Demonstrates file-based workflows:
- Loading text from files
- Processing multiple corpora
- Memory-efficient options
- Temporary file handling

```bash
cargo run --example file_processing
```

### 8. Tweet Generator (`tweet_generator.rs`)
Creative writing example:
- Twitter-length content (280 chars)
- Themed tweet generation
- Batch generation tips

```bash
cargo run --example tweet_generator
```

### 9. Email Generator (`email_generator.rs`)
Test data generation example:
- Character-level chain for names
- Multiple generation strategies
- Pattern-based email creation
- Bulk test data generation

```bash
cargo run --example email_generator
```

## Example Output

### Basic Example
```
=== Markov-RS Basic Usage Example ===

Creating model from corpus...
Model created successfully!

Generating 5 random sentences:
----------------------------------------
1. Holmes was known for his brilliant deductive reasoning.
2. Watson documented their adventures in various stories.
3. The detective was also skilled in chemistry and anatomy.
4. Holmes often played the violin when deep in thought.
5. Holmes and Watson became one of literature's famous duos.
```

### Tweet Generator
```
=== Tweet Generator Example ===

Generating tech tweets (max 280 chars):

Tweet 1:
----------------------------------------
Coffee: the most important tool in a developer's toolkit.
Length: 67 chars | 12 words
Remaining: 213 chars
```

## Creating Your Own Examples

1. Create a new file in `examples/`:
   ```rust
   //! My Example
   //!
   //! Run with: cargo run --example my_example

   use markovify_rs::Text;

   fn main() {
       let corpus = "Your text here.";
       let model = Text::new(corpus, 2, true, true, None).unwrap();
       
       if let Some(sentence) = model.make_sentence(None, None, None, None, None, None, None) {
           println!("{}", sentence);
       }
   }
   ```

2. Run it:
   ```bash
   cargo run --example my_example
   ```

## Dependencies

All examples use only the core `markov-rs` crate. No additional dependencies required.

## Tips

- **State Size**: Use 2 for general text, 3 for more coherent (but less varied) output
- **Compilation**: Always compile models for production use
- **Memory**: Set `retain_original=false` for large corpora
- **Overlap**: Disable with `test_output=false` for faster generation

## License

Same as the main project (MIT).
