//! Utility functions for combining models

use crate::chain::Chain;
use crate::errors::{MarkovError, Result};
use crate::text::Text;
use fxhash::FxHashMap;

/// Combine multiple models into one
///
/// # Arguments
/// * `models` - A list of models to combine (Chain or Text)
/// * `weights` - Optional weights for each model (default: equal weights)
///
/// # Returns
/// A combined model of the same type as the input models
pub fn combine_chains(models: Vec<&Chain>, weights: Option<Vec<f64>>) -> Result<Chain> {
    if models.is_empty() {
        return Err(MarkovError::CombineError("No models provided".to_string()));
    }

    let weights = weights.unwrap_or_else(|| vec![1.0; models.len()]);

    if models.len() != weights.len() {
        return Err(MarkovError::CombineError(
            "Models and weights lengths must be equal".to_string(),
        ));
    }

    // Check that all models have the same state size
    let state_size = models[0].state_size();
    for model in &models[1..] {
        if model.state_size() != state_size {
            return Err(MarkovError::CombineError(
                "All models must have the same state size".to_string(),
            ));
        }
    }

    // Check that no model is compiled
    for model in &models {
        if model.is_compiled() {
            return Err(MarkovError::CombineError(
                "Cannot combine compiled models".to_string(),
            ));
        }
    }

    // Combine the models by merging their model HashMaps
    let mut combined: FxHashMap<Vec<String>, FxHashMap<String, usize>> = FxHashMap::default();

    for (model, &weight) in models.iter().zip(weights.iter()) {
        for (state, options) in model.model() {
            let entry = combined.entry(state.clone()).or_default();
            for (next_word, &count) in options {
                let prev_count = entry.get(next_word).unwrap_or(&0);
                let new_count = *prev_count + (count as f64 * weight).round() as usize;
                entry.insert(next_word.clone(), new_count);
            }
        }
    }

    // Create a new chain directly with the combined model
    Ok(Chain::from_combined_model(combined, state_size))
}

/// Combine multiple Text models
pub fn combine_texts(models: Vec<&Text>, weights: Option<Vec<f64>>) -> Result<Text> {
    if models.is_empty() {
        return Err(MarkovError::CombineError("No models provided".to_string()));
    }

    let weights = weights.unwrap_or_else(|| vec![1.0; models.len()]);

    if models.len() != weights.len() {
        return Err(MarkovError::CombineError(
            "Models and weights lengths must be equal".to_string(),
        ));
    }

    // Check that all models have the same state size
    let state_size = models[0].state_size();
    for model in &models[1..] {
        if model.state_size() != state_size {
            return Err(MarkovError::CombineError(
                "All models must have the same state size".to_string(),
            ));
        }
    }

    // Check that no model is compiled
    for model in &models {
        if model.chain().is_compiled() {
            return Err(MarkovError::CombineError(
                "Cannot combine compiled models".to_string(),
            ));
        }
    }

    // Combine the underlying chains
    let chains: Vec<&Chain> = models.iter().map(|m| m.chain()).collect();
    let combined_chain = combine_chains(chains, Some(weights.clone()))?;

    // Combine parsed sentences if any model retains original
    let combined_sentences = if models.iter().any(|m| m.retain_original()) {
        let mut all_sentences = Vec::new();
        for model in models {
            if model.retain_original() {
                if let Some(sentences) = model.parsed_sentences() {
                    all_sentences.extend(sentences.iter().cloned());
                }
            }
        }
        Some(all_sentences)
    } else {
        None
    };

    Ok(Text::from_chain(
        combined_chain,
        combined_sentences.clone(),
        combined_sentences.is_some(),
    ))
}

/// Helper enum for combining different model types
pub enum ModelRef<'a> {
    Chain(&'a Chain),
    Text(&'a Text),
}

impl<'a> ModelRef<'a> {
    fn chain(&self) -> &Chain {
        match self {
            ModelRef::Chain(c) => c,
            ModelRef::Text(t) => t.chain(),
        }
    }
}

/// Combine models of potentially different types
pub fn combine_models(models: Vec<ModelRef>, weights: Option<Vec<f64>>) -> Result<CombinedResult> {
    if models.is_empty() {
        return Err(MarkovError::CombineError("No models provided".to_string()));
    }

    let weights = weights.unwrap_or_else(|| vec![1.0; models.len()]);

    if models.len() != weights.len() {
        return Err(MarkovError::CombineError(
            "Models and weights lengths must be equal".to_string(),
        ));
    }

    // Check state sizes match
    let state_size = models[0].chain().state_size();

    for model in &models[1..] {
        let model_state_size = model.chain().state_size();
        if model_state_size != state_size {
            return Err(MarkovError::CombineError(
                "All models must have the same state size".to_string(),
            ));
        }
    }

    // Check types match
    let first_is_chain = matches!(models[0], ModelRef::Chain(_));
    for model in &models[1..] {
        let is_chain = matches!(model, ModelRef::Chain(_));
        if is_chain != first_is_chain {
            return Err(MarkovError::CombineError(
                "All models must be of the same type".to_string(),
            ));
        }
    }

    // Check no model is compiled
    for model in &models {
        let is_compiled = model.chain().is_compiled();
        if is_compiled {
            return Err(MarkovError::CombineError(
                "Cannot combine compiled models".to_string(),
            ));
        }
    }

    // Combine based on type
    if first_is_chain {
        let chains: Vec<&Chain> = models
            .iter()
            .filter_map(|m| match m {
                ModelRef::Chain(c) => Some(*c),
                _ => None,
            })
            .collect();
        Ok(CombinedResult::Chain(combine_chains(
            chains,
            Some(weights),
        )?))
    } else {
        let texts: Vec<&Text> = models
            .iter()
            .filter_map(|m| match m {
                ModelRef::Text(t) => Some(*t),
                _ => None,
            })
            .collect();
        Ok(CombinedResult::Text(combine_texts(texts, Some(weights))?))
    }
}

/// Result of combining models
pub enum CombinedResult {
    Chain(Chain),
    Text(Text),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_chains_equal_weights() {
        let corpus1 = vec![vec!["hello".to_string(), "world".to_string()]];
        let corpus2 = vec![vec!["hello".to_string(), "rust".to_string()]];
        let chain1 = Chain::new(&corpus1, 1);
        let chain2 = Chain::new(&corpus2, 1);

        let combined = combine_chains(vec![&chain1, &chain2], None).unwrap();
        assert_eq!(combined.state_size(), 1);
    }

    #[test]
    fn test_combine_texts() {
        let text1 = "Hello world. Goodbye world.";
        let text2 = "Hello rust. Goodbye rust.";
        let model1 = Text::new(text1, 2, true, true, None).unwrap();
        let model2 = Text::new(text2, 2, true, true, None).unwrap();

        let combined = combine_texts(vec![&model1, &model2], None).unwrap();
        assert_eq!(combined.state_size(), 2);
    }

    #[test]
    fn test_combine_mismatched_state_sizes() {
        let corpus1 = vec![vec!["hello".to_string()]];
        let corpus2 = vec![vec!["hello".to_string(), "world".to_string()]];
        let chain1 = Chain::new(&corpus1, 1);
        let chain2 = Chain::new(&corpus2, 2);

        let result = combine_chains(vec![&chain1, &chain2], None);
        assert!(result.is_err());
    }
}
