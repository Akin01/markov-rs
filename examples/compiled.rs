//! Compiled Model Performance Example
//!
//! This example demonstrates the performance benefits of compiling models.
//! Compilation converts frequency dictionaries to cumulative frequency lists
//! for faster random selection during sentence generation.
//!
//! Run with: cargo run --example compiled

use markovify_rs::Text;
use std::time::Instant;

fn main() {
    // Larger corpus for meaningful benchmarks
    let corpus = r#"
        The morning sun cast long shadows across the ancient courtyard.
        Birds sang melodiously in the nearby oak trees.
        A gentle breeze rustled the leaves of the garden plants.
        The fountain in the center created a soothing sound.
        Children played happily on the grassy lawn nearby.
        An old man sat on a bench reading his newspaper.
        The smell of fresh coffee drifted from a nearby cafe.
        Cars passed by on the busy street beyond the park.
        A dog chased its tail while its owner laughed.
        The clock tower chimed loudly marking the hour.
        Students walked briskly towards the university campus.
        The library doors opened to welcome early visitors.
        A street performer played guitar for passing crowds.
        The aroma of baked goods filled the morning air.
        People hurried along the sidewalks to their destinations.
        The city was awakening to another beautiful day.
        Clouds drifted lazily across the bright blue sky.
        A butterfly landed gently on a colorful flower.
        The world continued its endless dance of life.
        Moments passed like grains of sand through time.
    "#
    .repeat(5); // Repeat to make corpus larger

    println!("=== Compiled Model Performance Example ===\n");

    // Create uncompiled model
    println!("Creating uncompiled model...");
    let start = Instant::now();
    let model = Text::new(&corpus, 2, true, true, None).expect("Failed to create model");
    let creation_time = start.elapsed();
    println!("  Creation time: {:.2?}\n", creation_time);

    // Compile the model
    println!("Compiling model...");
    let start = Instant::now();
    let compiled_model = model.compile();
    let compile_time = start.elapsed();
    println!("  Compile time: {:.2?}\n", compile_time);

    // Benchmark uncompiled generation
    let iterations = 100;
    println!("Generating {} sentences (uncompiled)...", iterations);
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = model.make_sentence(None, None, None, None, None, None, None);
    }
    let uncompiled_time = start.elapsed();
    let uncompiled_per_sec = iterations as f64 / uncompiled_time.as_secs_f64();
    println!(
        "  Time: {:.2?} ({:.0} sentences/sec)",
        uncompiled_time, uncompiled_per_sec
    );
    println!("  Avg per sentence: {:.2?}\n", uncompiled_time / iterations);

    // Benchmark compiled generation
    println!("Generating {} sentences (compiled)...", iterations);
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = compiled_model.make_sentence(None, None, None, None, None, None, None);
    }
    let compiled_time = start.elapsed();
    let compiled_per_sec = iterations as f64 / compiled_time.as_secs_f64();
    println!(
        "  Time: {:.2?} ({:.0} sentences/sec)",
        compiled_time, compiled_per_sec
    );
    println!("  Avg per sentence: {:.2?}\n", compiled_time / iterations);

    // Calculate speedup
    let speedup = uncompiled_time.as_secs_f64() / compiled_time.as_secs_f64();
    println!("Performance Improvement:");
    println!("{}", "-".repeat(40));
    println!("  Speedup factor: {:.2}x", speedup);
    println!("  Time saved: {:.2?}", uncompiled_time - compiled_time);
    println!("  Uncompiled: {:.0} sent/sec", uncompiled_per_sec);
    println!("  Compiled:   {:.0} sent/sec", compiled_per_sec);

    // Sample output
    println!("\nSample generated sentences:");
    println!("{}", "-".repeat(40));
    for i in 1..=3 {
        if let Some(sentence) =
            compiled_model.make_sentence(None, None, None, None, None, None, None)
        {
            println!("{}. {}", i, sentence);
        }
    }

    println!("\n=== Example Complete ===");
    println!("\nTip: Always compile models for production use!");
}
