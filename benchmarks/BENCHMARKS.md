# Benchmarking markov-rs

This document describes how to benchmark the Rust implementation (markov-rs) against the Python implementation (markovify).

## Overview

The benchmark suite compares:
- **Model Creation** - Time to build a Markov model from text
- **Sentence Generation** - Time to generate random sentences
- **Model Compilation** - Time to compile models for faster generation
- **JSON Serialization** - Time to save/load models
- **Model Combination** - Time to merge multiple models

## Quick Start

### Run All Benchmarks

```bash
# Using the runner script (recommended)
cd benchmarks
./run_benchmarks.sh --iterations 10

# Or run separately
# Rust benchmarks
cargo bench

# Python benchmarks
python3 benchmarks/python_benchmark.py
```

### Run Specific Benchmarks

```bash
# Rust only
./benchmarks/run_benchmarks.sh --rust-only

# Python only
./benchmarks/run_benchmarks.sh --python-only

# With custom iterations
./benchmarks/run_benchmarks.sh --iterations 50 --output results.json
```

## Requirements

### Rust Benchmarks
- Rust 1.70+ (with cargo)
- criterion 0.5 (automatically installed as dev dependency)

### Python Benchmarks
- Python 3.7+
- markovify (`pip install markovify`)

## Running Rust Benchmarks

```bash
cargo bench
```

This runs the criterion benchmarks and produces:
- Console output with timing statistics
- HTML report in `target/criterion/report/index.html`
- Raw data in `target/criterion/`

### Benchmark Categories

1. **model_creation** - Creating Text models from corpus
2. **sentence_generation** - Generating sentences (compiled vs uncompiled)
3. **model_compilation** - Compiling models for performance
4. **json_serialization** - Saving/loading models
5. **model_combination** - Merging multiple models
6. **chain_operations** - Low-level chain operations
7. **newline_text** - Newline-delimited text handling
8. **throughput** - Sentences per second

## Running Python Benchmarks

```bash
python3 benchmarks/python_benchmark.py --iterations 10
```

Options:
- `--iterations, -i N` - Number of iterations (default: 10)
- `--compare, -c` - Also run Rust benchmarks
- `--output, -o FILE` - Save results to JSON

## Interpreting Results

### Rust Criterion Output

```
model_creation/medium_corpus_state_2
                        time:   [2.3456 ms 2.3789 ms 2.4123 ms]
                        change: [-1.2345% -0.5678% +0.1234%] (p = 0.78 > 0.05)
                        No change in performance detected.
```

- **time**: Confidence interval (min, mean, max)
- **change**: Performance change from previous run
- **p value**: Statistical significance

### Python Output

```
Python markovify Benchmarks
============================================================
[1] Model Creation (Medium Corpus)
    Avg: 45.67 ms
```

## Performance Tips

### Rust Optimizations

1. **Use compiled models** - 2-5x faster for generation
2. **Reduce state_size** - Lower state sizes are faster
3. **Disable overlap checking** - Set `test_output=false`

```rust
let model = Text::new(&corpus, 2, true, true, None).unwrap().compile();
let sentence = model.make_sentence(
    None, None, None, None, Some(false), None, None
);
```

### Python Optimizations

1. **Use compiled models**
2. **Set retain_original=False** for large corpora
3. **Cache models** using JSON serialization

```python
model = markovify.Text(text).compile()
sentence = model.make_sentence(test_output=False)
```

## Expected Results

Typical performance characteristics (varies by hardware):

| Operation | Python (ms) | Rust (ms) | Speedup |
|-----------|-------------|-----------|---------|
| Model Creation (medium) | 50-100 | 5-15 | 5-10x |
| Sentence Generation | 1-5 | 0.01-0.1 | 50-100x |
| Compiled Generation | 0.5-2 | 0.01-0.05 | 20-50x |
| Model Compilation | 10-30 | 1-5 | 5-10x |
| JSON Serialize | 5-15 | 1-3 | 3-5x |
| JSON Deserialize | 10-25 | 2-5 | 4-6x |
| Model Combination | 20-50 | 2-8 | 5-10x |

**Note**: Rust shows the biggest improvements in sentence generation due to:
- No GIL (Global Interpreter Lock)
- Efficient memory management
- Optimized random number generation
- Compiled native code

## Troubleshooting

### Rust benchmarks fail to compile

```bash
cargo clean
cargo update
cargo bench
```

### Python benchmarks show import error

```bash
pip install markovify
# Or
pip3 install markovify
```

### Benchmarks timeout

Reduce iterations:
```bash
./benchmarks/run_benchmarks.sh --iterations 5
```

### Memory issues with large corpora

Use smaller test corpus or reduce iterations.

## Benchmark Files

```
benchmarks/
├── benchmarks.rs      # Rust criterion benchmarks
├── python_benchmark.py # Python benchmark script
├── run_benchmarks.sh  # Combined runner script
└── BENCHMARKS.md      # This documentation
```

## CI/CD Integration

Add to your CI pipeline:

```yaml
# GitHub Actions example
- name: Run Rust Benchmarks
  run: cargo bench -- --save-baseline main

- name: Run Python Benchmarks
  run: python3 benchmarks/python_benchmark.py --output py_results.json
```

## Contributing

When adding new features, please:
1. Add corresponding benchmarks
2. Document performance characteristics
3. Note any regressions in the changelog

## License

Same as the main project (MIT).
