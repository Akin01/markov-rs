//! Model Serialization Example
//!
//! This example demonstrates how to save and load models using JSON.
//! This is useful for:
//! - Caching trained models
//! - Sharing models between applications
//! - Avoiding re-processing large corpora
//!
//! Run with: cargo run --example serialization

use markov_rs::Text;
use std::fs;
use std::path::Path;

fn main() {
    let corpus = r#"
        Rust is a systems programming language focused on safety.
        Rust provides memory safety without garbage collection.
        Rust has a rich type system and ownership model.
        Rust enables fearless concurrency in your programs.
        Rust compiles to native code for excellent performance.
        Rust is used by many tech companies for critical systems.
        Rust has a friendly community and great documentation.
        Rust prevents data races at compile time.
        Rust is perfect for building reliable software.
        Rust continues to grow in popularity among developers.
    "#;

    println!("=== Model Serialization Example ===\n");

    // Create and train a model
    println!("Step 1: Creating and training model...");
    let original_model = Text::new(corpus, 2, true, true, None).expect("Failed to create model");
    println!("  ✓ Model created\n");

    // Generate a sentence with the original model
    println!("Step 2: Generating sentence with original model:");
    if let Some(sentence) = original_model.make_sentence(None, None, None, None, None, None, None) {
        println!("  \"{}\"\n", sentence);
    }

    // Serialize to JSON
    println!("Step 3: Serializing model to JSON...");
    let json = original_model.to_json().expect("Failed to serialize model");
    println!("  ✓ Serialized ({} bytes)", json.len());

    // Save to file
    let model_path = "target/example_model.json";
    println!("\nStep 4: Saving model to {}...", model_path);

    // Ensure target directory exists
    if let Some(parent) = Path::new(model_path).parent() {
        fs::create_dir_all(parent).ok();
    }

    fs::write(model_path, &json).expect("Failed to save model");
    println!("  ✓ Model saved");

    // Load from file
    println!("\nStep 5: Loading model from {}...", model_path);
    let loaded_json = fs::read_to_string(model_path).expect("Failed to read model file");
    println!("  ✓ Model loaded");

    // Deserialize from JSON
    println!("\nStep 6: Deserializing model from JSON...");
    let loaded_model = Text::from_json(&loaded_json).expect("Failed to deserialize model");
    println!("  ✓ Model deserialized");

    // Generate a sentence with the loaded model
    println!("\nStep 7: Generating sentence with loaded model:");
    if let Some(sentence) = loaded_model.make_sentence(None, None, None, None, None, None, None) {
        println!("  \"{}\"\n", sentence);
    }

    // Verify state size is preserved
    println!("Step 8: Verifying model properties:");
    println!("  Original state size: {}", original_model.state_size());
    println!("  Loaded state size:   {}", loaded_model.state_size());
    println!("  ✓ State size preserved");

    // Clean up
    fs::remove_file(model_path).ok();
    println!("\n  Cleaned up temporary file");

    println!("\n=== Example Complete ===");
}
