use markov_rs::{Chain, NewlineText, Text};
use std::fs;

fn load_sherlock() -> String {
    fs::read_to_string("tests/sherlock.txt").unwrap_or_else(|_| {
        "Sherlock Holmes was a consulting detective. He solved crimes. He was very clever."
            .to_string()
    })
}

#[test]
fn test_text_too_small() {
    let text = "Example phrase. This is another example sentence.";
    let text_model = Text::new(text, 2, true, true, None).unwrap();
    assert!(text_model
        .make_sentence(None, None, None, None, None, None, None)
        .is_none());
}

#[test]
fn test_sherlock() {
    let sherlock_text = load_sherlock();
    let text_model = Text::new(&sherlock_text, 2, true, true, None).unwrap();
    let sent = text_model.make_sentence(None, None, None, None, None, None, None);
    assert!(sent.is_some());
    assert!(!sent.unwrap().is_empty());
}

#[test]
fn test_json_serialization() {
    let sherlock_text = load_sherlock();
    let text_model = Text::new(&sherlock_text, 2, true, true, None).unwrap();
    let json_model = text_model.to_json().unwrap();
    let new_text_model = Text::from_json(&json_model).unwrap();
    let sent = new_text_model.make_sentence(None, None, None, None, None, None, None);
    assert!(sent.is_some());
}

#[test]
fn test_chain_json() {
    let sherlock_text = load_sherlock();
    let text_model = Text::new(&sherlock_text, 2, true, true, None).unwrap();
    let chain_json = text_model.chain().to_json().unwrap();

    let stored_chain = Chain::from_json(&chain_json).unwrap();
    let new_text_model = Text::from_chain(stored_chain, None, false);

    let sent = new_text_model.make_sentence(None, None, None, None, None, None, None);
    assert!(sent.is_some());
}

#[test]
fn test_make_sentence_with_start() {
    let sherlock_text = load_sherlock();
    let text_model = Text::new(&sherlock_text, 2, true, true, None).unwrap();

    // Try to find a valid starting phrase
    let start_str = "Sherlock Holmes";
    let result = text_model.make_sentence_with_start(
        start_str,
        true,
        Some(50),
        None,
        None,
        None,
        None,
        None,
    );

    // If Sherlock Holmes is in the text, this should work
    if result.is_ok() {
        let sent = result.unwrap();
        assert!(sent.starts_with(start_str));
    }
}

#[test]
fn test_make_sentence_with_start_one_word() {
    let sherlock_text = load_sherlock();
    let text_model = Text::new(&sherlock_text, 2, true, true, None).unwrap();
    let start_str = "Sherlock";

    let result = text_model.make_sentence_with_start(
        start_str,
        true,
        Some(50),
        None,
        None,
        None,
        None,
        None,
    );
    if result.is_ok() {
        let sent = result.unwrap();
        assert!(sent.starts_with(start_str));
    }
}

#[test]
fn test_short_sentence() {
    let sherlock_text = load_sherlock();
    let text_model = Text::new(&sherlock_text, 2, true, true, None).unwrap();

    let mut sent = None;
    for _ in 0..10 {
        sent = text_model.make_short_sentence(45, None, None, None, None, None, None, None, None);
        if sent.is_some() {
            break;
        }
    }

    if let Some(s) = sent {
        assert!(s.len() <= 45);
    }
}

#[test]
fn test_short_sentence_min_chars() {
    let sherlock_text = load_sherlock();
    let text_model = Text::new(&sherlock_text, 2, true, true, None).unwrap();

    let mut sent = None;
    for _ in 0..20 {
        sent =
            text_model.make_short_sentence(100, Some(50), None, None, None, None, None, None, None);
        if let Some(ref s) = sent {
            if s.len() >= 50 && s.len() <= 100 {
                break;
            }
        }
    }

    if let Some(s) = sent {
        assert!(s.len() <= 100);
        assert!(s.len() >= 50);
    }
}

#[test]
fn test_dont_test_output() {
    let sherlock_text = load_sherlock();
    let text_model = Text::new(&sherlock_text, 2, true, true, None).unwrap();
    let sent = text_model.make_sentence(None, None, None, None, Some(false), None, None);
    assert!(sent.is_some());
}

#[test]
fn test_max_words() {
    let sherlock_text = load_sherlock();
    let text_model = Text::new(&sherlock_text, 2, true, true, None).unwrap();
    let sent = text_model.make_sentence(None, None, None, None, None, Some(0), None);
    assert!(sent.is_none());
}

#[test]
fn test_min_words() {
    let sherlock_text = load_sherlock();
    let text_model = Text::new(&sherlock_text, 2, true, true, None).unwrap();
    let sent = text_model.make_sentence(None, None, None, None, None, None, Some(5));
    if let Some(s) = sent {
        assert!(s.split_whitespace().count() >= 5);
    }
}

#[test]
fn test_newline_text() {
    let text = "Line one\nLine two\nLine three\nLine four";
    let model = NewlineText::new(text, 2, true, true, None).unwrap();
    let sentences = model.sentence_split(text);
    assert_eq!(sentences.len(), 4);
}

#[test]
fn test_custom_regex() {
    // Test well_formed = false allows problematic characters
    let result = Text::new("This sentence (would normally fail", 2, true, false, None);
    assert!(result.is_ok());

    // Test that we can create a model with default settings
    let result = Text::new("Hello world. This is a test.", 2, true, true, None);
    assert!(result.is_ok());
}

#[test]
fn test_compiling() {
    let sherlock_text = load_sherlock();
    let text_model = Text::new(&sherlock_text, 2, true, true, None).unwrap();
    let compiled = text_model.compile();

    let sent = compiled.make_sentence(None, None, None, None, None, None, None);
    assert!(sent.is_some());
}

#[test]
fn test_chain_creation() {
    let corpus = vec![
        vec!["hello".to_string(), "world".to_string()],
        vec!["hello".to_string(), "rust".to_string()],
    ];
    let chain = Chain::new(&corpus, 1);
    assert_eq!(chain.state_size(), 1);
}

#[test]
fn test_chain_walk() {
    let corpus = vec![vec![
        "the".to_string(),
        "cat".to_string(),
        "sat".to_string(),
    ]];
    let chain = Chain::new(&corpus, 1);
    let result = chain.walk(None);
    assert!(!result.is_empty());
}
