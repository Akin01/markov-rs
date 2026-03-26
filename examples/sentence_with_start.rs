//! Sentence Generation with Specific Start
//!
//! This example shows how to generate sentences that begin with specific words.
//!
//! Run with: cargo run --example sentence_with_start

use markovify_rs::Text;

fn main() {
    let corpus = r#"
        The quick brown fox jumps over the lazy dog.
        The quick runner sprinted across the field.
        The lazy cat slept on the warm windowsill.
        The brown bear wandered through the forest.
        The dog barked loudly at the passing car.
        The forest was filled with tall green trees.
        The field stretched endlessly toward the horizon.
        The windowsill provided a perfect napping spot.
        The car sped down the winding country road.
        The fox was clever and hunted at night.
    "#;

    println!("=== Sentence Generation with Specific Start ===\n");

    let model = Text::new(corpus, 2, true, true, None).expect("Failed to create model");

    // Generate sentences starting with specific words
    let starters = vec!["The quick", "The lazy", "The brown", "The", "fox"];

    for starter in starters {
        println!("Trying to generate sentence starting with: \"{}\"", starter);

        match model.make_sentence_with_start(
            starter,
            true,     // strict mode - only sentences starting with these words
            Some(20), // try 20 times
            None,
            None,
            None,
            None,
            None,
        ) {
            Ok(sentence) => {
                println!("  ✓ Success: {}", sentence);
            }
            Err(e) => {
                println!("  ✗ Failed: {}", e);

                // Try non-strict mode (finds sentences containing the words)
                if starter.split_whitespace().count() == 1 {
                    println!("  Trying non-strict mode...");
                    if let Ok(sentence) = model.make_sentence_with_start(
                        starter,
                        false, // non-strict - sentences containing the word
                        Some(20),
                        None,
                        None,
                        None,
                        None,
                        None,
                    ) {
                        println!("  ✓ Success (non-strict): {}", sentence);
                    }
                }
            }
        }
        println!();
    }

    println!("=== Example Complete ===");
}
