//! File-Based Text Processing Example
//!
//! This example demonstrates how to:
//! - Load text from files
//! - Process large corpora efficiently
//! - Save and load models
//!
//! Run with: cargo run --example file_processing
//!
//! Note: This example creates temporary files in the target directory.

use markov_rs::Text;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("=== File Processing Example ===\n");

    // Create sample corpus files
    let target_dir = Path::new("target/examples");
    fs::create_dir_all(target_dir)?;

    let corpus1_path = target_dir.join("corpus1.txt");
    let corpus2_path = target_dir.join("corpus2.txt");
    let model_path = target_dir.join("model.json");

    // Write sample corpora
    let corpus1 = r#"
        Technology continues to advance at a rapid pace.
        Artificial intelligence is transforming many industries.
        Machine learning enables computers to learn from data.
        Cloud computing provides scalable infrastructure.
        Cybersecurity remains a critical concern for all.
        Software development practices evolve constantly.
        Open source collaboration drives innovation forward.
        Mobile devices have become essential tools daily.
        The Internet connects billions of people worldwide.
        Digital transformation affects every business sector.
    "#;

    let corpus2 = r#"
        Science helps us understand the natural world better.
        Research leads to new discoveries and breakthroughs.
        Experiments test hypotheses and validate theories.
        Data analysis reveals patterns and insights hidden.
        Collaboration accelerates scientific progress greatly.
        Peer review ensures quality and accuracy maintained.
        Funding supports important research initiatives globally.
        Education prepares the next generation scientists.
        Curiosity drives the pursuit of knowledge always.
        Innovation transforms scientific findings applications.
    "#;

    println!("Step 1: Writing corpus files...");
    fs::write(&corpus1_path, corpus1)?;
    fs::write(&corpus2_path, corpus2)?;
    println!("  ✓ Created {}", corpus1_path.display());
    println!("  ✓ Created {}", corpus2_path.display());
    println!();

    // Load and process first corpus
    println!("Step 2: Loading and processing corpus 1...");
    let text1 = fs::read_to_string(&corpus1_path)?;
    let model1 = Text::new(&text1, 2, true, true, None).expect("Failed to create model 1");
    println!("  ✓ Model 1 created");

    if let Some(sentence) = model1.make_sentence(None, None, None, None, None, None, None) {
        println!("  Sample: {}", sentence);
    }
    println!();

    // Load and process second corpus
    println!("Step 3: Loading and processing corpus 2...");
    let text2 = fs::read_to_string(&corpus2_path)?;
    let model2 = Text::new(&text2, 2, true, true, None).expect("Failed to create model 2");
    println!("  ✓ Model 2 created");

    if let Some(sentence) = model2.make_sentence(None, None, None, None, None, None, None) {
        println!("  Sample: {}", sentence);
    }
    println!();

    // Combine models
    println!("Step 4: Combining models...");
    let combined = markov_rs::utils::combine_texts(vec![&model1, &model2], None)
        .expect("Failed to combine models");
    println!("  ✓ Models combined");

    if let Some(sentence) = combined.make_sentence(None, None, None, None, None, None, None) {
        println!("  Sample: {}", sentence);
    }
    println!();

    // Save combined model
    println!("Step 5: Saving combined model to JSON...");
    let json = combined.to_json().expect("Failed to serialize model");
    fs::write(&model_path, &json)?;
    println!("  ✓ Model saved ({} bytes)", json.len());
    println!();

    // Load model back
    println!("Step 6: Loading model from JSON...");
    let loaded_json = fs::read_to_string(&model_path)?;
    let loaded_model = Text::from_json(&loaded_json).expect("Failed to load model");
    println!("  ✓ Model loaded");

    if let Some(sentence) = loaded_model.make_sentence(None, None, None, None, None, None, None) {
        println!("  Sample: {}", sentence);
    }
    println!();

    // Generate multiple sentences
    println!("Step 7: Generating 5 sentences from loaded model:");
    println!("{}", "-".repeat(40));
    for i in 1..=5 {
        if let Some(sentence) = loaded_model.make_sentence(None, None, None, None, None, None, None)
        {
            println!("{}. {}", i, sentence);
        }
    }
    println!();

    // Memory-efficient processing for large files
    println!("Step 8: Memory-efficient processing tip:");
    println!("{}", "-".repeat(40));
    println!("For very large files, use retain_original=false:");
    println!("  let model = Text::new(&large_text, 2, false, true, None);");
    println!("This reduces memory usage but disables overlap checking.");
    println!();

    // Cleanup
    println!("Step 9: Cleaning up temporary files...");
    fs::remove_file(&corpus1_path)?;
    fs::remove_file(&corpus2_path)?;
    fs::remove_file(&model_path)?;
    println!("  ✓ Files removed");

    println!("\n=== Example Complete ===");

    Ok(())
}
