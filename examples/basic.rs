//! Basic Usage Example
//!
//! This example demonstrates the fundamental usage of markov-rs:
//! - Creating a text model
//! - Generating random sentences
//! - Generating short sentences
//!
//! Run with: cargo run --example basic

use markovify_rs::Text;

fn main() {
    // Sample corpus - in practice, you'd load this from a file
    // Using a larger corpus for better sentence generation
    let corpus = r#"
        Sherlock Holmes was a consulting detective in London.
        He lived at 221B Baker Street with his friend Dr. Watson.
        Holmes was known for his brilliant deductive reasoning.
        He solved many challenging cases throughout his career.
        Watson documented their adventures in various stories.
        The detective was also skilled in chemistry and anatomy.
        Holmes often played the violin when deep in thought.
        His arch-nemesis was the criminal mastermind Moriarty.
        The detective stories were written by Arthur Conan Doyle.
        Holmes and Watson became one of literature's famous duos.
        The Baker Street residence was their shared apartment.
        Dr. Watson was a former army doctor with experience.
        Holmes used scientific methods to solve mysteries.
        Many clients came to consult with the detective.
        Scotland Yard often sought Holmes assistance with cases.
        The detective had a keen eye for small details.
        Watson admired his friend's remarkable intellectual powers.
        Moriarty was a professor of mathematics at university.
        The criminal underworld feared the great detective.
        London streets provided many mysteries to investigate.
        Holmes preferred logic over emotional considerations.
        The game was afoot whenever a new case arrived.
        Watson's medical knowledge sometimes proved useful.
        Mrs. Hudson was their landlady at Baker Street.
        The detective could disappear in disguise easily.
    "#;

    println!("=== Markov-RS Basic Usage Example ===\n");

    // Create a model with state size 2 (default)
    println!("Creating model from corpus...");
    let model = Text::new(corpus, 2, true, true, None).expect("Failed to create model");

    println!("Model created successfully!\n");

    // Generate 5 random sentences
    println!("Generating 5 random sentences:");
    println!("{}", "-".repeat(40));
    for i in 1..=5 {
        if let Some(sentence) = model.make_sentence(None, None, None, None, None, None, None) {
            println!("{}. {}", i, sentence);
        } else {
            println!("{}. (no sentence generated)", i);
        }
    }
    println!();

    // Generate 3 short sentences (max 80 characters)
    println!("Generating 3 short sentences (max 80 chars):");
    println!("{}", "-".repeat(40));
    for i in 1..=3 {
        if let Some(sentence) =
            model.make_short_sentence(80, None, None, None, None, None, None, None, None)
        {
            println!("{}. {} ({} chars)", i, sentence, sentence.len());
        }
    }
    println!();

    // Generate a sentence with specific constraints
    println!("Generating sentence with 5-15 words:");
    println!("{}", "-".repeat(40));
    if let Some(sentence) = model.make_sentence(
        None,       // no specific start
        Some(20),   // try up to 20 times
        None,       // default overlap ratio
        None,       // default overlap total
        Some(true), // test output for overlap
        Some(15),   // max 15 words
        Some(5),    // min 5 words
    ) {
        println!("Generated: {}", sentence);
        println!("Word count: {}", sentence.split_whitespace().count());
    } else {
        println!("Could not generate sentence with constraints");
    }

    println!("\n=== Example Complete ===");
}
