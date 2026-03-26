//! Text processing and sentence generation

use crate::chain::{Chain, BEGIN};
use crate::errors::{MarkovError, Result};
use crate::splitters::split_into_sentences;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Default maximum overlap ratio for sentence output testing
const DEFAULT_MAX_OVERLAP_RATIO: f64 = 0.7;
/// Default maximum overlap total for sentence output testing
const DEFAULT_MAX_OVERLAP_TOTAL: usize = 15;
/// Default number of tries for sentence generation
const DEFAULT_TRIES: usize = 10;

lazy_static! {
    /// Pattern to reject sentences with problematic characters
    static ref REJECT_PAT: Regex = Regex::new(r#"(^')|('$)|\s'|'\s|["(\(\)\[\])]"#).unwrap();
    /// Pattern for splitting words
    static ref WORD_SPLIT_PATTERN: Regex = Regex::new(r"\s+").unwrap();
}

/// Serialized representation of Text for JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextData {
    pub state_size: usize,
    pub chain: String, // JSON string of the chain
    pub parsed_sentences: Option<Vec<Vec<String>>>,
}

/// A Markov chain text model for generating random sentences
#[derive(Debug, Clone)]
pub struct Text {
    state_size: usize,
    chain: Chain,
    parsed_sentences: Option<Vec<Vec<String>>>,
    rejoined_text: Option<String>,
    retain_original: bool,
    well_formed: bool,
    reject_pat: Regex,
}

impl Text {
    /// Create a new Text model from input text
    ///
    /// # Arguments
    /// * `input_text` - The source text to build the model from
    /// * `state_size` - Number of words in the model's state (default: 2)
    /// * `retain_original` - Whether to keep the original corpus for overlap checking
    /// * `well_formed` - Whether to reject sentences with unmatched quotes/parenthesis
    /// * `reject_reg` - Optional custom regex pattern for rejecting sentences
    pub fn new(
        input_text: &str,
        state_size: usize,
        retain_original: bool,
        well_formed: bool,
        reject_reg: Option<&str>,
    ) -> Result<Self> {
        let reject_pat = if let Some(reg) = reject_reg {
            Regex::new(reg).map_err(|e| MarkovError::ParamError(format!("Invalid regex: {}", e)))?
        } else {
            REJECT_PAT.clone()
        };

        let parsed_sentences: Vec<Vec<String>> =
            Self::generate_corpus(input_text, &reject_pat, well_formed)
                .into_iter()
                .collect();

        let rejoined_text = if retain_original && !parsed_sentences.is_empty() {
            Some(Self::sentence_join_static(
                &parsed_sentences
                    .iter()
                    .map(|s| Self::word_join_static(s))
                    .collect::<Vec<_>>(),
            ))
        } else {
            None
        };

        let chain = Chain::new(&parsed_sentences, state_size);

        Ok(Text {
            state_size,
            chain,
            parsed_sentences: if retain_original {
                Some(parsed_sentences)
            } else {
                None
            },
            rejoined_text,
            retain_original,
            well_formed,
            reject_pat,
        })
    }

    /// Create a Text model from an existing chain
    pub fn from_chain(
        chain: Chain,
        parsed_sentences: Option<Vec<Vec<String>>>,
        retain_original: bool,
    ) -> Self {
        let state_size = chain.state_size();

        let rejoined_text = if retain_original {
            parsed_sentences.as_ref().map(|sentences| {
                Self::sentence_join_static(
                    &sentences
                        .iter()
                        .map(|s| Self::word_join_static(s))
                        .collect::<Vec<_>>(),
                )
            })
        } else {
            None
        };

        Text {
            state_size,
            chain,
            parsed_sentences,
            rejoined_text,
            retain_original,
            well_formed: true,
            reject_pat: REJECT_PAT.clone(),
        }
    }

    /// Split text into sentences
    pub fn sentence_split(&self, text: &str) -> Vec<String> {
        split_into_sentences(text)
    }

    /// Join sentences into text
    pub fn sentence_join(&self, sentences: &[String]) -> String {
        sentences.join(" ")
    }

    /// Split a sentence into words
    pub fn word_split(&self, sentence: &str) -> Vec<String> {
        WORD_SPLIT_PATTERN
            .split(sentence)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    /// Join words into a sentence
    pub fn word_join(&self, words: &[String]) -> String {
        words.join(" ")
    }

    /// Test if a sentence input is valid
    pub fn test_sentence_input(&self, sentence: &str) -> bool {
        if sentence.trim().is_empty() {
            return false;
        }

        if self.well_formed && self.reject_pat.is_match(sentence) {
            return false;
        }

        true
    }

    /// Generate a corpus from text
    fn generate_corpus(text: &str, reject_pat: &Regex, well_formed: bool) -> Vec<Vec<String>> {
        let sentences = split_into_sentences(text);

        sentences
            .into_iter()
            .filter(|s| {
                if !well_formed {
                    return true;
                }
                // Test sentence input
                if s.trim().is_empty() {
                    return false;
                }
                if reject_pat.is_match(s) {
                    return false;
                }
                true
            })
            .map(|s| {
                WORD_SPLIT_PATTERN
                    .split(&s)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect()
            })
            .collect()
    }

    /// Test if a generated sentence output is acceptable
    fn test_sentence_output(
        &self,
        words: &[String],
        max_overlap_ratio: f64,
        max_overlap_total: usize,
    ) -> bool {
        if let Some(ref rejoined) = self.rejoined_text {
            let overlap_ratio = ((max_overlap_ratio * words.len() as f64).round() as usize).max(1);
            let overlap_max = overlap_ratio.min(max_overlap_total);
            let overlap_over = overlap_max + 1;
            let gram_count = words.len().saturating_sub(overlap_max).max(1);

            for i in 0..gram_count {
                let gram = &words[i..(i + overlap_over).min(words.len())];
                let gram_joined = self.word_join(gram);
                if rejoined.contains(&gram_joined) {
                    return false;
                }
            }
        }
        true
    }

    /// Generate a random sentence
    ///
    /// # Arguments
    /// * `init_state` - Optional starting state (tuple of words)
    /// * `tries` - Maximum number of attempts (default: 10)
    /// * `max_overlap_ratio` - Maximum overlap ratio with original text (default: 0.7)
    /// * `max_overlap_total` - Maximum overlap total with original text (default: 15)
    /// * `test_output` - Whether to test output for overlap (default: true)
    /// * `max_words` - Maximum number of words in the sentence
    /// * `min_words` - Minimum number of words in the sentence
    #[allow(clippy::too_many_arguments)]
    pub fn make_sentence(
        &self,
        init_state: Option<&[String]>,
        tries: Option<usize>,
        max_overlap_ratio: Option<f64>,
        max_overlap_total: Option<usize>,
        test_output: Option<bool>,
        max_words: Option<usize>,
        min_words: Option<usize>,
    ) -> Option<String> {
        let tries = tries.unwrap_or(DEFAULT_TRIES);
        let mor = max_overlap_ratio.unwrap_or(DEFAULT_MAX_OVERLAP_RATIO);
        let mot = max_overlap_total.unwrap_or(DEFAULT_MAX_OVERLAP_TOTAL);
        let test = test_output.unwrap_or(true);

        let prefix: Vec<String> = if let Some(state) = init_state {
            state.iter().filter(|w| *w != BEGIN).cloned().collect()
        } else {
            vec![]
        };

        for _ in 0..tries {
            let mut words = prefix.clone();
            words.extend(self.chain.walk(init_state));

            // Check word count constraints
            if let Some(max) = max_words {
                if words.len() > max {
                    continue;
                }
            }
            if let Some(min) = min_words {
                if words.len() < min {
                    continue;
                }
            }

            // Test output if required
            if test && self.rejoined_text.is_some() {
                if self.test_sentence_output(&words, mor, mot) {
                    return Some(self.word_join(&words));
                }
            } else {
                return Some(self.word_join(&words));
            }
        }

        None
    }

    /// Generate a short sentence with a maximum character count
    #[allow(clippy::too_many_arguments)]
    pub fn make_short_sentence(
        &self,
        max_chars: usize,
        min_chars: Option<usize>,
        init_state: Option<&[String]>,
        tries: Option<usize>,
        max_overlap_ratio: Option<f64>,
        max_overlap_total: Option<usize>,
        test_output: Option<bool>,
        max_words: Option<usize>,
        min_words: Option<usize>,
    ) -> Option<String> {
        let tries = tries.unwrap_or(DEFAULT_TRIES);
        let min_chars = min_chars.unwrap_or(0);

        for _ in 0..tries {
            if let Some(sentence) = self.make_sentence(
                init_state,
                Some(tries),
                max_overlap_ratio,
                max_overlap_total,
                test_output,
                max_words,
                min_words,
            ) {
                let len = sentence.len();
                if len >= min_chars && len <= max_chars {
                    return Some(sentence);
                }
            }
        }

        None
    }

    /// Generate a sentence that starts with a specific string
    #[allow(clippy::too_many_arguments)]
    pub fn make_sentence_with_start(
        &self,
        beginning: &str,
        strict: bool,
        tries: Option<usize>,
        max_overlap_ratio: Option<f64>,
        max_overlap_total: Option<usize>,
        test_output: Option<bool>,
        max_words: Option<usize>,
        min_words: Option<usize>,
    ) -> Result<String> {
        let split = self.word_split(beginning);
        let word_count = split.len();

        if word_count > self.state_size {
            return Err(MarkovError::ParamError(format!(
                "`make_sentence_with_start` for this model requires a string containing 1 to {} words. Yours has {}: {:?}",
                self.state_size, word_count, split
            )));
        }

        let init_states: Vec<Vec<String>> = if word_count == self.state_size {
            vec![split.clone()]
        } else if word_count < self.state_size {
            if strict {
                // Pad with BEGIN tokens
                let mut state = vec![BEGIN.to_string(); self.state_size - word_count];
                state.extend(split.clone());
                vec![state]
            } else {
                // Find all chains containing this sequence
                self.find_init_states_from_chain(&split)
            }
        } else {
            return Err(MarkovError::ParamError(format!(
                "Invalid word count: {}",
                word_count
            )));
        };

        if init_states.is_empty() {
            return Err(MarkovError::ParamError(format!(
                "Cannot find sentence beginning with: {}",
                beginning
            )));
        }

        // Try each init state
        for init_state in init_states {
            if let Some(output) = self.make_sentence(
                Some(&init_state),
                tries,
                max_overlap_ratio,
                max_overlap_total,
                test_output,
                max_words,
                min_words,
            ) {
                return Ok(output);
            }
        }

        Err(MarkovError::ParamError(format!(
            "Cannot generate sentence beginning with: {}",
            beginning
        )))
    }

    /// Find all initial states from the chain that contain the given split
    fn find_init_states_from_chain(&self, split: &[String]) -> Vec<Vec<String>> {
        let word_count = split.len();
        let mut states = Vec::new();

        for key in self.chain.model().keys() {
            // Filter out BEGIN tokens and check if it starts with split
            let filtered: Vec<&String> = key.iter().filter(|w| *w != BEGIN).collect();
            if filtered.len() >= word_count
                && filtered[..word_count]
                    .iter()
                    .zip(split.iter())
                    .all(|(a, b)| *a == b)
            {
                states.push(key.clone());
            }
        }

        states
    }

    /// Compile the model for faster generation
    pub fn compile(&self) -> Self {
        let compiled_chain = self.chain.compile();

        Text {
            state_size: self.state_size,
            chain: compiled_chain,
            parsed_sentences: self.parsed_sentences.clone(),
            rejoined_text: self.rejoined_text.clone(),
            retain_original: self.retain_original,
            well_formed: self.well_formed,
            reject_pat: self.reject_pat.clone(),
        }
    }

    /// Compile the model in place (returns self)
    pub fn compile_inplace(&mut self) {
        self.chain = self.chain.compile();
    }

    /// Get the state size
    pub fn state_size(&self) -> usize {
        self.state_size
    }

    /// Get the chain
    pub fn chain(&self) -> &Chain {
        &self.chain
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String> {
        let data = TextData {
            state_size: self.state_size,
            chain: self.chain.to_json()?,
            parsed_sentences: self.parsed_sentences.clone(),
        };
        Ok(serde_json::to_string(&data)?)
    }

    /// Deserialize from JSON
    pub fn from_json(json_str: &str) -> Result<Self> {
        let data: TextData = serde_json::from_str(json_str)?;
        let chain = Chain::from_json(&data.chain)?;

        Ok(Text {
            state_size: data.state_size,
            chain,
            parsed_sentences: data.parsed_sentences.clone(),
            rejoined_text: data.parsed_sentences.as_ref().map(|sentences| {
                Self::sentence_join_static(
                    &sentences
                        .iter()
                        .map(|s| Self::word_join_static(s))
                        .collect::<Vec<_>>(),
                )
            }),
            retain_original: data.parsed_sentences.is_some(),
            well_formed: true,
            reject_pat: REJECT_PAT.clone(),
        })
    }

    /// Check if the model retains original sentences
    pub fn retain_original(&self) -> bool {
        self.retain_original
    }

    /// Get the parsed sentences if available
    pub fn parsed_sentences(&self) -> Option<&Vec<Vec<String>>> {
        self.parsed_sentences.as_ref()
    }

    fn sentence_join_static(sentences: &[String]) -> String {
        sentences.join(" ")
    }

    fn word_join_static(words: &[String]) -> String {
        words.join(" ")
    }
}

/// A text model that splits on newlines instead of sentence punctuation
#[derive(Debug, Clone)]
pub struct NewlineText {
    inner: Text,
}

impl NewlineText {
    /// Create a new NewlineText model
    pub fn new(
        input_text: &str,
        state_size: usize,
        retain_original: bool,
        well_formed: bool,
        reject_reg: Option<&str>,
    ) -> Result<Self> {
        let text = Text::new(
            input_text,
            state_size,
            retain_original,
            well_formed,
            reject_reg,
        )?;
        Ok(NewlineText { inner: text })
    }

    /// Split text on newlines
    pub fn sentence_split(&self, text: &str) -> Vec<String> {
        text.split('\n')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Generate a sentence
    #[allow(clippy::too_many_arguments)]
    pub fn make_sentence(
        &self,
        init_state: Option<&[String]>,
        tries: Option<usize>,
        max_overlap_ratio: Option<f64>,
        max_overlap_total: Option<usize>,
        test_output: Option<bool>,
        max_words: Option<usize>,
        min_words: Option<usize>,
    ) -> Option<String> {
        self.inner.make_sentence(
            init_state,
            tries,
            max_overlap_ratio,
            max_overlap_total,
            test_output,
            max_words,
            min_words,
        )
    }

    /// Generate a short sentence
    #[allow(clippy::too_many_arguments)]
    pub fn make_short_sentence(
        &self,
        max_chars: usize,
        min_chars: Option<usize>,
        init_state: Option<&[String]>,
        tries: Option<usize>,
        max_overlap_ratio: Option<f64>,
        max_overlap_total: Option<usize>,
        test_output: Option<bool>,
        max_words: Option<usize>,
        min_words: Option<usize>,
    ) -> Option<String> {
        self.inner.make_short_sentence(
            max_chars,
            min_chars,
            init_state,
            tries,
            max_overlap_ratio,
            max_overlap_total,
            test_output,
            max_words,
            min_words,
        )
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String> {
        self.inner.to_json()
    }

    /// Deserialize from JSON
    pub fn from_json(json_str: &str) -> Result<Self> {
        let text = Text::from_json(json_str)?;
        Ok(NewlineText { inner: text })
    }

    /// Get the inner Text model
    pub fn inner(&self) -> &Text {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_creation() {
        let text = "Hello world. This is a test.";
        let model = Text::new(text, 2, true, true, None).unwrap();
        assert_eq!(model.state_size(), 2);
    }

    #[test]
    fn test_make_sentence() {
        // Need more text for state_size=2 to work properly
        let text = "The cat sat on the mat. The dog ran in the park. The bird flew over the tree. The cat chased the mouse. The dog barked loudly.";
        let model = Text::new(text, 1, true, true, None).unwrap();
        let sentence = model.make_sentence(None, None, None, None, None, None, None);
        assert!(sentence.is_some());
    }

    #[test]
    fn test_json_serialization() {
        let text = "Hello world. This is a test.";
        let model = Text::new(text, 2, true, true, None).unwrap();
        let json = model.to_json().unwrap();
        let restored = Text::from_json(&json).unwrap();
        assert_eq!(model.state_size(), restored.state_size());
    }

    #[test]
    fn test_newline_text() {
        let text = "Line one
Line two
Line three";
        let model = NewlineText::new(text, 2, true, true, None).unwrap();
        let sentences = model.sentence_split(text);
        assert_eq!(sentences.len(), 3);
    }
}
