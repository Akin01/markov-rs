//! Model Combination Example
//!
//! This example demonstrates how to combine multiple models with different weights.
//! Useful for:
//! - Merging models from different sources
//! - Creating hybrid writing styles
//! - Incremental model updates
//!
//! Run with: cargo run --example combine

use markovify_rs::{utils::combine_texts, Text};

fn main() {
    // Two different writing styles
    let formal_corpus = r#"
        The committee has reviewed the proposal thoroughly.
        The findings indicate significant potential for growth.
        The methodology employed was rigorous and systematic.
        The results demonstrate clear improvements in efficiency.
        The recommendations have been carefully considered.
        The implementation requires additional resources allocation.
        The stakeholders have expressed their support unanimously.
        The timeline appears feasible given current constraints.
    "#;

    let casual_corpus = r#"
        Hey, so I was thinking about this cool idea.
        You know what would be really awesome though.
        Like, imagine if we could just make it happen.
        Honestly, I think it's gonna work out great.
        Anyway, let's catch up later about this stuff.
        Pretty sure everyone's gonna love the results.
        Just wanted to share my thoughts on everything.
        Cool, so yeah, that's pretty much the plan.
    "#;

    println!("=== Model Combination Example ===\n");

    // Create two separate models
    println!("Creating two models with different styles...");
    let formal_model =
        Text::new(formal_corpus, 2, true, true, None).expect("Failed to create formal model");
    let casual_model =
        Text::new(casual_corpus, 2, true, true, None).expect("Failed to create casual model");
    println!("  ✓ Formal model created");
    println!("  ✓ Casual model created\n");

    // Generate from each model separately
    println!("Generating from formal model:");
    println!("{}", "-".repeat(40));
    for _ in 1..=2 {
        if let Some(sentence) = formal_model.make_sentence(None, None, None, None, None, None, None)
        {
            println!("  {}", sentence);
        }
    }
    println!();

    println!("Generating from casual model:");
    println!("{}", "-".repeat(40));
    for _ in 1..=2 {
        if let Some(sentence) = casual_model.make_sentence(None, None, None, None, None, None, None)
        {
            println!("  {}", sentence);
        }
    }
    println!();

    // Combine with equal weights
    println!("Combining models with equal weights (50/50)...");
    let combined_equal =
        combine_texts(vec![&formal_model, &casual_model], None).expect("Failed to combine models");
    println!("  ✓ Models combined\n");

    println!("Generating from combined model (equal weights):");
    println!("{}", "-".repeat(40));
    for _ in 1..=3 {
        if let Some(sentence) =
            combined_equal.make_sentence(None, None, None, None, None, None, None)
        {
            println!("  {}", sentence);
        }
    }
    println!();

    // Combine with weighted emphasis on formal
    println!("Combining models with formal emphasis (70/30)...");
    let combined_formal = combine_texts(
        vec![&formal_model, &casual_model],
        Some(vec![2.0, 1.0]), // Formal gets 2x weight
    )
    .expect("Failed to combine models");
    println!("  ✓ Models combined\n");

    println!("Generating from combined model (formal weighted):");
    println!("{}", "-".repeat(40));
    for _ in 1..=3 {
        if let Some(sentence) =
            combined_formal.make_sentence(None, None, None, None, None, None, None)
        {
            println!("  {}", sentence);
        }
    }
    println!();

    // Combine with weighted emphasis on casual
    println!("Combining models with casual emphasis (30/70)...");
    let combined_casual = combine_texts(
        vec![&formal_model, &casual_model],
        Some(vec![1.0, 2.0]), // Casual gets 2x weight
    )
    .expect("Failed to combine models");

    println!("Generating from combined model (casual weighted):");
    println!("{}", "-".repeat(40));
    for _ in 1..=3 {
        if let Some(sentence) =
            combined_casual.make_sentence(None, None, None, None, None, None, None)
        {
            println!("  {}", sentence);
        }
    }

    println!("\n=== Example Complete ===");
}
