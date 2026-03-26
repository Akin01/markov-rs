//! Newline-Delimited Text Example
//!
//! This example shows how to use NewlineText for text where sentences
//! are separated by newlines instead of punctuation.
//!
//! Useful for:
//! - Poetry
//! - Song lyrics
//! - One sentence per line corpora
//! - Chat logs
//!
//! Run with: cargo run --example newline_text

use markovify_rs::NewlineText;

fn main() {
    // Poetry corpus (each line is a separate unit)
    let poetry = r#"
Roses are red and beautiful to see
Violets are blue and sweet as can be
The sun shines bright on a summer day
Children laugh and love to play
The moon glows soft in the night sky
Stars twinkle as the clouds drift by
The ocean waves crash on the shore
Seagulls cry and then soar some more
The wind whispers through the tall trees
Leaves dance gently in the breeze
Morning dew sparkles on the grass
Time passes like sand in glass
The world turns and seasons change
Nothing stays quite the same
Life goes on in its own way
Each brings a brand new day
    "#;

    // Chat log style corpus
    let chat_log = r#"
hey how are you doing today
i am doing great thanks for asking
that is wonderful to hear
yeah been having a good week
what about you anything new
just working on some projects
sounds interesting tell me more
it is a machine learning thing
oh cool that sounds awesome
trying to learn rust programming
rust is really great for performance
yeah that is what I heard
the community is super helpful too
definitely check out the docs
will do thanks for the tip
    "#;

    println!("=== NewlineText Example ===\n");

    // Poetry model
    println!("1. Poetry Model");
    println!("{}", "-".repeat(40));
    let poetry_model =
        NewlineText::new(poetry, 2, true, true, None).expect("Failed to create poetry model");

    println!("Generated poetry lines:");
    for i in 1..=5 {
        if let Some(line) = poetry_model.make_sentence(None, None, None, None, None, None, None) {
            println!("  {}: {}", i, line);
        }
    }
    println!();

    // Chat model
    println!("2. Chat Log Model");
    println!("{}", "-".repeat(40));
    let chat_model =
        NewlineText::new(chat_log, 2, true, true, None).expect("Failed to create chat model");

    println!("Generated chat messages:");
    for i in 1..=5 {
        if let Some(msg) = chat_model.make_sentence(None, None, None, None, None, None, None) {
            println!("  {}: {}", i, msg);
        }
    }
    println!();

    // Short lines (max 40 chars)
    println!("3. Short Poetry Lines (max 40 chars):");
    println!("{}", "-".repeat(40));
    for i in 1..=3 {
        if let Some(line) =
            poetry_model.make_short_sentence(40, None, None, None, None, None, None, None, None)
        {
            println!("  {}: {} ({} chars)", i, line, line.len());
        }
    }

    println!("\n=== Example Complete ===");
    println!("\nNote: NewlineText splits on newlines instead of");
    println!("sentence punctuation, making it perfect for poetry,");
    println!("lyrics, chat logs, and other line-based text.");
}
