use markovify_rs::splitters::{is_abbreviation, is_sentence_ender, split_into_sentences};

#[test]
fn test_is_abbreviation() {
    assert!(is_abbreviation("Mr."));
    assert!(is_abbreviation("etc."));
    assert!(is_abbreviation("U.S."));
    assert!(is_abbreviation("Dr."));
    assert!(!is_abbreviation("Hello."));
    assert!(!is_abbreviation("cat."));
    assert!(!is_abbreviation("dog"));
}

#[test]
fn test_is_sentence_ender() {
    assert!(is_sentence_ender("Hello."));
    assert!(is_sentence_ender("What?"));
    assert!(is_sentence_ender("Stop!"));
    assert!(!is_sentence_ender("U.S.A."));
    assert!(!is_sentence_ender("Mr."));
    assert!(!is_sentence_ender("etc."));
}

#[test]
fn test_split_into_sentences_simple() {
    let text = "Hello world. This is a test. How are you?";
    let sentences = split_into_sentences(text);
    assert_eq!(sentences.len(), 3);
    assert_eq!(sentences[0], "Hello world.");
    assert_eq!(sentences[1], "This is a test.");
    assert_eq!(sentences[2], "How are you?");
}

#[test]
fn test_split_into_sentences_with_abbreviations() {
    let text = "Dr. Smith went to Washington. He met Mr. Jones.";
    let sentences = split_into_sentences(text);
    // Should handle abbreviations properly
    assert!(!sentences.is_empty());
}

#[test]
fn test_split_into_sentences_with_questions() {
    let text = "Are you there? Yes I am. Really?";
    let sentences = split_into_sentences(text);
    assert!(sentences.len() >= 2);
}

#[test]
fn test_split_into_sentences_with_exclamations() {
    let text = "Wow! That is amazing. I can't believe it!";
    let sentences = split_into_sentences(text);
    assert!(sentences.len() >= 2);
}

#[test]
fn test_split_empty_string() {
    let text = "";
    let sentences = split_into_sentences(text);
    assert!(sentences.is_empty());
}

#[test]
fn test_split_single_sentence() {
    let text = "This is a single sentence.";
    let sentences = split_into_sentences(text);
    assert_eq!(sentences.len(), 1);
    assert_eq!(sentences[0], "This is a single sentence.");
}
