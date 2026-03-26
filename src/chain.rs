//! Markov chain implementation

use crate::errors::{MarkovError, Result};
use fxhash::FxHashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Special token indicating the beginning of a sequence
pub const BEGIN: &str = "___BEGIN__";
/// Special token indicating the end of a sequence
pub const END: &str = "___END__";

/// Cumulative frequency data for efficient random selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledNext {
    pub words: Vec<String>,
    pub cumulative_weights: Vec<usize>,
}

/// A Markov chain representing processes that have both beginnings and ends.
///
/// The chain is represented as a HashMap where keys are states (tuples of words)
/// and values are HashMaps of possible next words with their frequencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    state_size: usize,
    model: FxHashMap<Vec<String>, FxHashMap<String, usize>>,
    compiled: bool,
    #[serde(skip)]
    compiled_model: FxHashMap<Vec<String>, CompiledNext>,
    #[serde(skip)]
    begin_choices: Option<Vec<String>>,
    #[serde(skip)]
    begin_cumdist: Option<Vec<usize>>,
}

impl Chain {
    /// Create a new Markov chain from a corpus
    ///
    /// # Arguments
    /// * `corpus` - A list of runs, where each run is a sequence of items (e.g., words in a sentence)
    /// * `state_size` - The number of items used to represent the chain's state
    pub fn new(corpus: &[Vec<String>], state_size: usize) -> Self {
        let model = Self::build(corpus, state_size);
        let mut chain = Chain {
            state_size,
            model,
            compiled: false,
            compiled_model: FxHashMap::default(),
            begin_choices: None,
            begin_cumdist: None,
        };
        chain.precompute_begin_state();
        chain
    }

    /// Build the Markov model from a corpus
    fn build(
        corpus: &[Vec<String>],
        state_size: usize,
    ) -> FxHashMap<Vec<String>, FxHashMap<String, usize>> {
        let mut model: FxHashMap<Vec<String>, FxHashMap<String, usize>> = FxHashMap::default();

        for run in corpus {
            let mut items: Vec<String> = vec![BEGIN.to_string(); state_size];
            items.extend(run.iter().cloned());
            items.push(END.to_string());

            for i in 0..=run.len() {
                let state: Vec<String> = items[i..i + state_size].to_vec();
                let follow = items[i + state_size].clone();

                let next_dict = model.entry(state).or_default();
                *next_dict.entry(follow).or_insert(0) += 1;
            }
        }

        model
    }

    /// Precompute the beginning state for faster sentence generation
    fn precompute_begin_state(&mut self) {
        let begin_state: Vec<String> = vec![BEGIN.to_string(); self.state_size];
        if let Some(next_dict) = self.model.get(&begin_state) {
            let (choices, cumdist) = Self::compile_next_dict(next_dict);
            self.begin_choices = Some(choices);
            self.begin_cumdist = Some(cumdist);
        }
    }

    /// Compile a next dictionary for efficient random selection
    fn compile_next_dict(next_dict: &FxHashMap<String, usize>) -> (Vec<String>, Vec<usize>) {
        let mut words = Vec::with_capacity(next_dict.len());
        let mut cumulative_weights = Vec::with_capacity(next_dict.len());
        let mut cumsum = 0;

        for (word, &weight) in next_dict.iter() {
            words.push(word.clone());
            cumsum += weight;
            cumulative_weights.push(cumsum);
        }

        (words, cumulative_weights)
    }

    /// Compile the chain for faster generation
    ///
    /// This converts the frequency dictionaries into cumulative frequency lists
    /// for more efficient random selection.
    pub fn compile(&self) -> Self {
        let mut compiled_model: FxHashMap<Vec<String>, CompiledNext> = FxHashMap::default();

        for (state, next_dict) in &self.model {
            let (words, cumulative_weights) = Self::compile_next_dict(next_dict);
            compiled_model.insert(
                state.clone(),
                CompiledNext {
                    words,
                    cumulative_weights,
                },
            );
        }

        Chain {
            state_size: self.state_size,
            model: self.model.clone(),
            compiled: true,
            compiled_model,
            begin_choices: self.begin_choices.clone(),
            begin_cumdist: self.begin_cumdist.clone(),
        }
    }

    /// Choose the next item given the current state
    fn move_state(&self, state: &[String]) -> Option<String> {
        let (choices, cumdist) = if self.compiled {
            if let Some(compiled) = self.compiled_model.get(state) {
                (&compiled.words, &compiled.cumulative_weights)
            } else {
                return None;
            }
        } else if state.iter().all(|s| s == BEGIN) {
            if let (Some(choices), Some(cumdist)) = (&self.begin_choices, &self.begin_cumdist) {
                (choices, cumdist)
            } else {
                return None;
            }
        } else {
            if let Some(next_dict) = self.model.get(state) {
                let (choices, cumdist) = Self::compile_next_dict(next_dict);
                // For uncompiled chains, we compute on the fly
                return Self::select_random(&choices, &cumdist);
            } else {
                return None;
            }
        };

        if cumdist.is_empty() {
            return None;
        }

        Self::select_random(choices, cumdist)
    }

    /// Select a random item based on cumulative weights
    fn select_random(choices: &[String], cumdist: &[usize]) -> Option<String> {
        if cumdist.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0..cumdist[cumdist.len() - 1]);

        // Binary search for the selection
        let idx = cumdist.partition_point(|&x| x <= r);

        if idx < choices.len() {
            Some(choices[idx].clone())
        } else {
            Some(choices[choices.len() - 1].clone())
        }
    }

    /// Generate items from the chain
    ///
    /// Returns an iterator that yields successive items until the END state is reached.
    pub fn gen(&self, init_state: Option<&[String]>) -> ChainGenerator<'_> {
        let state = init_state
            .map(|s| s.to_vec())
            .unwrap_or_else(|| vec![BEGIN.to_string(); self.state_size]);

        ChainGenerator {
            chain: self,
            state,
            done: false,
        }
    }

    /// Walk the chain and return a complete sequence
    ///
    /// Returns a vector representing a single run of the Markov model.
    pub fn walk(&self, init_state: Option<&[String]>) -> Vec<String> {
        self.gen(init_state).collect()
    }

    /// Get the state size
    pub fn state_size(&self) -> usize {
        self.state_size
    }

    /// Get the model (for inspection or combination)
    pub fn model(&self) -> &FxHashMap<Vec<String>, FxHashMap<String, usize>> {
        &self.model
    }

    /// Check if the chain is compiled
    pub fn is_compiled(&self) -> bool {
        self.compiled
    }

    /// Serialize the chain to JSON
    pub fn to_json(&self) -> Result<String> {
        let items: Vec<(Vec<String>, FxHashMap<String, usize>)> = self
            .model
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        Ok(serde_json::to_string(&items)?)
    }

    /// Deserialize a chain from JSON
    pub fn from_json(json_str: &str) -> Result<Self> {
        let items: Vec<(Vec<String>, FxHashMap<String, usize>)> = serde_json::from_str(json_str)?;

        if items.is_empty() {
            return Err(MarkovError::ModelFormatError("Empty model".to_string()));
        }

        let state_size = items[0].0.len();
        let model: FxHashMap<Vec<String>, FxHashMap<String, usize>> = items.into_iter().collect();

        let mut chain = Chain {
            state_size,
            model,
            compiled: false,
            compiled_model: FxHashMap::default(),
            begin_choices: None,
            begin_cumdist: None,
        };
        chain.precompute_begin_state();
        Ok(chain)
    }

    /// Create a chain from a pre-built model (used for combining models)
    pub fn from_combined_model(
        model: FxHashMap<Vec<String>, FxHashMap<String, usize>>,
        state_size: usize,
    ) -> Self {
        let mut chain = Chain {
            state_size,
            model,
            compiled: false,
            compiled_model: FxHashMap::default(),
            begin_choices: None,
            begin_cumdist: None,
        };
        chain.precompute_begin_state();
        chain
    }
}

/// Iterator for generating sequences from a Markov chain
pub struct ChainGenerator<'a> {
    chain: &'a Chain,
    state: Vec<String>,
    done: bool,
}

impl<'a> Iterator for ChainGenerator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if let Some(next_word) = self.chain.move_state(&self.state) {
            if next_word == END {
                self.done = true;
                return None;
            }

            // Update state
            self.state.remove(0);
            self.state.push(next_word.clone());
            Some(next_word)
        } else {
            self.done = true;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_chain_json_serialization() {
        let corpus = vec![vec!["hello".to_string(), "world".to_string()]];
        let chain = Chain::new(&corpus, 1);
        let json = chain.to_json().unwrap();
        let restored = Chain::from_json(&json).unwrap();
        assert_eq!(chain.state_size(), restored.state_size());
    }
}
