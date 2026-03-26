//! Benchmarks for markov-rs vs markovify (Python)
//!
//! Run with: cargo bench
//!
//! This benchmark suite tests:
//! - Model creation time
//! - Sentence generation time
//! - Model compilation time
//! - Model combination time
//! - JSON serialization/deserialization

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use markovify_rs::{Chain, NewlineText, Text};
use std::fs;

/// Load the Sherlock Holmes corpus
#[allow(dead_code)]
fn load_sherlock_corpus() -> String {
    fs::read_to_string("tests/sherlock.txt").unwrap_or_else(|_| {
        // Fallback corpus if file doesn't exist
        let base =
            "Sherlock Holmes was a consulting detective. He lived at 221B Baker Street in London. ";
        base.repeat(500)
    })
}

/// Load a small corpus for quick benchmarks
fn load_small_corpus() -> String {
    "The cat sat on the mat. The dog ran in the park. The bird flew over the tree. ".repeat(50)
        + "The sun was shining brightly. The children played in the garden. "
        + "The teacher explained the lesson. The students listened carefully. "
        + "The chef prepared a delicious meal. The guests enjoyed the food. "
        + "The artist painted a beautiful picture. The audience admired the work. "
        + "The musician played a wonderful song. The crowd applauded enthusiastically. "
        + "The writer composed an interesting story. The readers found it captivating. "
        + "The scientist conducted an important experiment. The results were significant. "
        + "The engineer designed an innovative solution. The project was successful. "
        + "The doctor treated the patient with care. The recovery was remarkable."
}

/// Load a medium corpus for realistic benchmarks
fn load_medium_corpus() -> String {
    fs::read_to_string("tests/sherlock.txt")
        .unwrap_or_else(|_| "Sherlock Holmes was a consulting detective. ".repeat(200))
}

// ============================================================================
// Model Creation Benchmarks
// ============================================================================

fn bench_model_creation(c: &mut Criterion) {
    let small_corpus = load_small_corpus();
    let medium_corpus = load_medium_corpus();

    let mut group = c.benchmark_group("model_creation");

    group.bench_function("small_corpus_state_2", |b| {
        b.iter(|| Text::new(black_box(&small_corpus), 2, true, true, None).unwrap())
    });

    group.bench_function("medium_corpus_state_2", |b| {
        b.iter(|| Text::new(black_box(&medium_corpus), 2, true, true, None).unwrap())
    });

    group.bench_function("medium_corpus_state_3", |b| {
        b.iter(|| Text::new(black_box(&medium_corpus), 3, true, true, None).unwrap())
    });

    group.finish();
}

// ============================================================================
// Sentence Generation Benchmarks
// ============================================================================

fn bench_sentence_generation(c: &mut Criterion) {
    let corpus = load_medium_corpus();
    let model = Text::new(&corpus, 2, true, true, None).unwrap();
    let compiled_model = model.compile();

    let mut group = c.benchmark_group("sentence_generation");

    group.bench_function("make_sentence_uncompiled", |b| {
        b.iter(|| model.make_sentence(None, Some(10), None, None, None, None, None))
    });

    group.bench_function("make_sentence_compiled", |b| {
        b.iter(|| compiled_model.make_sentence(None, Some(10), None, None, None, None, None))
    });

    group.bench_function("make_short_sentence_100_chars", |b| {
        b.iter(|| {
            compiled_model.make_short_sentence(100, None, None, None, None, None, None, None, None)
        })
    });

    group.bench_function("make_sentence_with_start", |b| {
        b.iter(|| {
            compiled_model.make_sentence_with_start(
                "Sherlock",
                true,
                Some(10),
                None,
                None,
                None,
                None,
                None,
            )
        })
    });

    group.finish();
}

// ============================================================================
// Model Compilation Benchmarks
// ============================================================================

fn bench_model_compilation(c: &mut Criterion) {
    let corpus = load_medium_corpus();
    let model = Text::new(&corpus, 2, true, true, None).unwrap();

    let mut group = c.benchmark_group("model_compilation");

    group.bench_function("compile_model", |b| b.iter(|| model.compile()));

    group.finish();
}

// ============================================================================
// JSON Serialization Benchmarks
// ============================================================================

fn bench_json_serialization(c: &mut Criterion) {
    let corpus = load_medium_corpus();
    let model = Text::new(&corpus, 2, true, true, None).unwrap();
    let json = model.to_json().unwrap();

    let mut group = c.benchmark_group("json_serialization");

    group.bench_function("to_json", |b| b.iter(|| model.to_json().unwrap()));

    group.bench_function("from_json", |b| {
        b.iter(|| Text::from_json(black_box(&json)).unwrap())
    });

    group.finish();
}

// ============================================================================
// Model Combination Benchmarks
// ============================================================================

fn bench_model_combination(c: &mut Criterion) {
    let corpus = load_medium_corpus();
    let model_a = Text::new(&corpus, 2, true, true, None).unwrap();
    let model_b = Text::new(&corpus, 2, true, true, None).unwrap();

    let mut group = c.benchmark_group("model_combination");

    group.bench_function("combine_two_models_equal_weights", |b| {
        b.iter(|| markovify_rs::utils::combine_texts(vec![&model_a, &model_b], None).unwrap())
    });

    group.bench_function("combine_two_models_weighted", |b| {
        b.iter(|| {
            markovify_rs::utils::combine_texts(vec![&model_a, &model_b], Some(vec![1.5, 1.0]))
                .unwrap()
        })
    });

    group.finish();
}

// ============================================================================
// Chain-Level Benchmarks
// ============================================================================

fn bench_chain_operations(c: &mut Criterion) {
    let corpus = vec![
        "the quick brown fox jumps over the lazy dog"
            .split_whitespace()
            .map(String::from)
            .collect(),
        "the fast red cat runs under the slow mouse"
            .split_whitespace()
            .map(String::from)
            .collect(),
    ];
    let chain = Chain::new(&corpus, 2);

    let mut group = c.benchmark_group("chain_operations");

    group.bench_function("chain_walk", |b| b.iter(|| chain.walk(None)));

    group.bench_function("chain_gen", |b| {
        b.iter(|| chain.gen(None).collect::<Vec<_>>())
    });

    group.bench_function("chain_compile", |b| b.iter(|| chain.compile()));

    group.finish();
}

// ============================================================================
// NewlineText Benchmarks
// ============================================================================

fn bench_newline_text(c: &mut Criterion) {
    let corpus: String = (1..=100)
        .map(|i| format!("This is line number {} in the corpus.", i))
        .collect::<Vec<_>>()
        .join("\n");

    let mut group = c.benchmark_group("newline_text");

    group.bench_function("create_newline_text", |b| {
        b.iter(|| NewlineText::new(black_box(&corpus), 2, true, true, None).unwrap())
    });

    let model = NewlineText::new(&corpus, 2, true, true, None).unwrap();

    group.bench_function("newline_make_sentence", |b| {
        b.iter(|| model.make_sentence(None, Some(10), None, None, None, None, None))
    });

    group.finish();
}

// ============================================================================
// Throughput Benchmarks (sentences per second)
// ============================================================================

fn bench_throughput(c: &mut Criterion) {
    let corpus = load_medium_corpus();
    let model = Text::new(&corpus, 2, true, true, None).unwrap().compile();

    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Elements(1));

    group.bench_function("sentences_per_second", |b| {
        b.iter(|| model.make_sentence(None, Some(10), None, None, Some(false), None, None))
    });

    group.finish();
}

// ============================================================================
// Main Benchmark Groups
// ============================================================================

criterion_group!(
    benches,
    bench_model_creation,
    bench_sentence_generation,
    bench_model_compilation,
    bench_json_serialization,
    bench_model_combination,
    bench_chain_operations,
    bench_newline_text,
    bench_throughput,
);

criterion_main!(benches);
