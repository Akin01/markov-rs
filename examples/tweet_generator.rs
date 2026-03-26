//! Creative Writing / Tweet Generator Example
//!
//! This example demonstrates how to use markov-rs for creative writing,
//! specifically generating tweet-like content from a corpus.
//!
//! Run with: cargo run --example tweet_generator

use markovify_rs::Text;

fn main() {
    // A corpus of tech-themed tweets
    let tech_tweets = r#"
        Just shipped a new feature to production!
        Debugging is like being a detective in a crime movie.
        Coffee: the most important tool in a developer's toolkit.
        Why do programmers prefer dark mode? Because light attracts bugs.
        Finally fixed that bug that's been haunting me for days.
        Reading documentation is a superpower in disguise.
        The best code is the code you don't have to write.
        Refactoring: making code cleaner without changing behavior.
        Git commit messages tell the story of your project.
        Testing is not optional, it's essential.
        Learning a new programming language expands your mind.
        Open source is built on collaboration and trust.
        The perfect is the enemy of the good.
        Ship early, ship often, iterate based on feedback.
        Code review is about improving code, not finding faults.
        Documentation is a love letter to your future self.
        Automation saves time and reduces human error.
        Performance optimization is premature if done too early.
        Security should be built in, not bolted on.
        The cloud makes scaling easier than ever before.
        APIs are contracts between services and clients.
        Databases are the backbone of most applications.
        Frontend development is both art and science.
        Backend systems power the digital world we live in.
        DevOps bridges development and operations teams.
    "#;

    println!("=== Tweet Generator Example ===\n");

    // Create model with state size 2 for coherent tweets
    let model = Text::new(tech_tweets, 2, true, true, None).expect("Failed to create model");

    // Generate tweet-length content (max 280 characters)
    println!("Generating tech tweets (max 280 chars):\n");

    for i in 1..=5 {
        if let Some(tweet) = model.make_short_sentence(
            280,      // max chars (Twitter limit)
            Some(50), // min chars
            None,     // no specific start
            Some(20), // try 20 times
            None,     // default overlap ratio
            None,     // default overlap total
            None,     // default test output
            None,     // no max words
            None,     // no min words
        ) {
            println!("Tweet {}:", i);
            println!("{}", "-".repeat(40));
            println!("{}", tweet);
            println!(
                "Length: {} chars | {} words",
                tweet.len(),
                tweet.split_whitespace().count()
            );

            // Show remaining chars
            let remaining = 280 - tweet.len();
            println!("Remaining: {} chars\n", remaining);
        } else {
            println!("Tweet {}: (could not generate)\n", i);
        }
    }

    // Generate tweets starting with specific phrases
    println!("\nGenerating tweets with specific starts:\n");

    let starters = vec!["Just", "Finally", "Why", "The best", "Code"];

    for starter in starters {
        if let Ok(tweet) = model.make_sentence_with_start(
            starter,
            true,     // strict mode
            Some(15), // try 15 times
            None,
            None,
            None,
            None,
            None,
        ) {
            if tweet.len() <= 280 {
                println!("Starting with '{}':", starter);
                println!("  {}", tweet);
                println!("  ({} chars)\n", tweet.len());
            }
        }
    }

    // Compile for faster generation
    println!("Compiling model for faster generation...");
    let compiled = model.compile();

    println!("\nGenerating 3 quick tweets (compiled model):");
    println!("{}", "-".repeat(40));
    for i in 1..=3 {
        if let Some(tweet) =
            compiled.make_short_sentence(200, None, None, None, None, None, None, None, None)
        {
            println!("{}. {}", i, tweet);
        }
    }

    println!("\n=== Example Complete ===");
    println!("\nTips for tweet generation:");
    println!("  • Use state_size=2 for coherent short text");
    println!("  • Set max_chars=280 for Twitter limit");
    println!("  • Compile model for faster batch generation");
    println!("  • Use make_sentence_with_start for themed tweets");
}
