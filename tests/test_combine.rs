use markovify_rs::utils::{combine_chains, combine_models, combine_texts, ModelRef};
use markovify_rs::Text;

fn load_sherlock() -> String {
    std::fs::read_to_string("tests/sherlock.txt")
        .unwrap_or_else(|_| "Sherlock Holmes was a consulting detective. He solved crimes. He was very clever. The game was afoot.".to_string())
}

#[test]
fn test_combine_simple() {
    let sherlock = load_sherlock();
    let text_model = Text::new(&sherlock, 2, true, true, None).unwrap();

    let combo = combine_texts(vec![&text_model, &text_model], Some(vec![0.5, 0.5])).unwrap();
    // Combined model should have same structure
    assert_eq!(combo.state_size(), 2);
}

#[test]
fn test_combine_double_weighted() {
    let sherlock = load_sherlock();
    let text_model = Text::new(&sherlock, 2, true, true, None).unwrap();

    let combo = combine_texts(vec![&text_model, &text_model], Some(vec![2.0, 1.0])).unwrap();
    assert_eq!(combo.state_size(), 2);
}

#[test]
fn test_combine_chains() {
    let sherlock = load_sherlock();
    let text_model = Text::new(&sherlock, 2, true, true, None).unwrap();
    let chain = text_model.chain();

    let result = combine_chains(vec![chain, chain], None);
    assert!(result.is_ok());
}

#[test]
fn test_combine_no_weights() {
    let sherlock = load_sherlock();
    let text_model = Text::new(&sherlock, 2, true, true, None).unwrap();

    let combo = combine_texts(vec![&text_model, &text_model], None).unwrap();
    assert_eq!(combo.state_size(), 2);
}

#[test]
fn test_combine_mismatched_state_sizes() {
    let sherlock = load_sherlock();
    let text_model_a = Text::new(&sherlock, 2, true, true, None).unwrap();
    let text_model_b = Text::new(&sherlock, 3, true, true, None).unwrap();

    let result = combine_texts(vec![&text_model_a, &text_model_b], None);
    assert!(result.is_err());
}

#[test]
fn test_combine_compiled_model_fail() {
    let sherlock = load_sherlock();
    let model_a = Text::new(&sherlock, 2, true, true, None).unwrap();
    let model_b = model_a.compile();

    let result = combine_texts(vec![&model_a, &model_b], None);
    assert!(result.is_err());
}

#[test]
fn test_combine_compiled_chain_fail() {
    let sherlock = load_sherlock();
    let model_a = Text::new(&sherlock, 2, true, true, None).unwrap();
    let model_b = model_a.compile();

    let result = combine_chains(vec![model_a.chain(), model_b.chain()], None);
    assert!(result.is_err());
}

#[test]
fn test_combine_no_retain() {
    let sherlock = load_sherlock();
    let text_model = Text::new(&sherlock, 2, false, true, None).unwrap();

    let combo = combine_texts(vec![&text_model, &text_model], None).unwrap();
    assert!(!combo.retain_original());
}

#[test]
fn test_combine_retain_on_no_retain() {
    let sherlock = load_sherlock();
    let text_model_a = Text::new(&sherlock, 2, false, true, None).unwrap();
    let text_model_b = Text::new(&sherlock, 2, true, true, None).unwrap();

    let combo = combine_texts(vec![&text_model_a, &text_model_b], None).unwrap();
    assert!(combo.retain_original());
}

#[test]
fn test_combine_no_retain_on_retain() {
    let sherlock = load_sherlock();
    let text_model_a = Text::new(&sherlock, 2, true, true, None).unwrap();
    let text_model_b = Text::new(&sherlock, 2, false, true, None).unwrap();

    let combo = combine_texts(vec![&text_model_a, &text_model_b], None).unwrap();
    assert!(combo.retain_original());
}

#[test]
fn test_combine_models_enum() {
    let sherlock = load_sherlock();
    let text_model = Text::new(&sherlock, 2, true, true, None).unwrap();

    let result = combine_models(
        vec![ModelRef::Text(&text_model), ModelRef::Text(&text_model)],
        Some(vec![1.0, 1.0]),
    );
    assert!(result.is_ok());
}

#[test]
fn test_combine_mismatched_types() {
    let sherlock = load_sherlock();
    let text_model = Text::new(&sherlock, 2, true, true, None).unwrap();
    let chain = text_model.chain();

    // Try to combine Text and Chain - should fail
    let result = combine_models(
        vec![ModelRef::Text(&text_model), ModelRef::Chain(chain)],
        None,
    );
    assert!(result.is_err());
}
