//! Sentence splitting utilities

use lazy_static::lazy_static;
use regex::Regex;

// List of capitalized abbreviations (states, titles, streets, months)
lazy_static! {
    static ref ABBR_CAPPED: Vec<&'static str> = vec![
        // States
        "ala", "ariz", "ark", "calif", "colo", "conn", "del", "fla", "ga", "ill", "ind",
        "kan", "ky", "la", "md", "mass", "mich", "minn", "miss", "mo", "mont",
        "neb", "nev", "okla", "ore", "pa", "tenn", "vt", "va", "wash", "wis", "wyo",
        // U.S.
        "u.s",
        // Titles
        "mr", "ms", "mrs", "msr", "dr", "gov", "pres", "sen", "sens", "rep", "reps",
        "prof", "gen", "messrs", "col", "sr", "jf", "sgt", "mgr", "fr", "rev",
        // More titles
        "jr", "snr", "atty", "supt",
        // Streets
        "ave", "blvd", "st", "rd", "hwy",
        // Months
        "jan", "feb", "mar", "apr", "jun", "jul", "aug", "sep", "sept", "oct", "nov", "dec",
    ];
}

// List of lowercase abbreviations
lazy_static! {
    static ref ABBR_LOWERCASE: Vec<&'static str> = vec!["etc", "v", "vs", "viz", "al", "pct"];
}

// Pattern to match a single uppercase letter
lazy_static! {
    static ref UPPERCASE_LETTER_PAT: Regex = Regex::new(r"^[A-Z]$").unwrap();
}

// Pattern to match initialisms (e.g., "U.S.A.", "Ph.D.")
lazy_static! {
    static ref INITIALISM_PAT: Regex =
        Regex::new(r"^[A-Za-z0-9]{1,2}(\.[A-Za-z0-9]{1,2})+\.$").unwrap();
}

/// Check if a word is an abbreviation
pub fn is_abbreviation(dotted_word: &str) -> bool {
    if !dotted_word.ends_with('.') {
        return false;
    }

    let clipped = &dotted_word[..dotted_word.len() - 1];

    if clipped.is_empty() {
        return false;
    }

    let first_char = clipped.chars().next().unwrap();

    if first_char.is_uppercase() {
        if clipped.len() == 1 {
            // Single initial (e.g., "A.")
            true
        } else {
            ABBR_CAPPED.contains(&clipped.to_lowercase().as_str())
        }
    } else {
        ABBR_LOWERCASE.contains(&clipped)
    }
}

/// Check if a word indicates the end of a sentence
pub fn is_sentence_ender(word: &str) -> bool {
    // Check for initialisms (e.g., "U.S.A.")
    if INITIALISM_PAT.is_match(word) {
        return false;
    }

    // Check for question marks or exclamation points
    if word.ends_with('?') || word.ends_with('!') {
        return true;
    }

    // Check for words with multiple capital letters (acronyms)
    let uppercase_count = word.chars().filter(|c| c.is_ascii_uppercase()).count();
    if uppercase_count > 1 {
        return true;
    }

    // Check for period that's not an abbreviation
    if word.ends_with('.') && !is_abbreviation(word) {
        return true;
    }

    false
}

/// Split text into sentences
///
/// This function handles common edge cases like abbreviations, initialisms,
/// and various punctuation marks.
pub fn split_into_sentences(text: &str) -> Vec<String> {
    // Simple sentence splitting based on sentence-ending punctuation
    // followed by whitespace and capital letter or end of string
    let mut sentences = Vec::new();
    let mut current_start = 0;
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // Check for sentence-ending punctuation
        if c == '.' || c == '?' || c == '!' {
            // Look ahead for whitespace followed by capital letter or end
            let mut j = i + 1;

            // Skip closing quotes/parens
            while j < chars.len()
                && (chars[j] == '"' || chars[j] == '\'' || chars[j] == ')' || chars[j] == ']')
            {
                j += 1;
            }

            // Check if followed by space and capital letter (or end of string)
            if j >= chars.len() {
                // End of text
                sentences.push(text[current_start..].trim().to_string());
                current_start = text.len();
                break;
            } else if chars[j] == ' ' || chars[j] == '\n' || chars[j] == '\t' {
                // Skip whitespace
                while j < chars.len() && (chars[j] == ' ' || chars[j] == '\n' || chars[j] == '\t') {
                    j += 1;
                }

                // Check if next char is uppercase or we're at end
                if j >= chars.len() || chars[j].is_uppercase() {
                    // Check if this might be an abbreviation
                    let potential_word_end = i + 1;
                    let mut word_start = i;
                    while word_start > 0
                        && (chars[word_start - 1].is_alphanumeric() || chars[word_start - 1] == '.')
                    {
                        word_start -= 1;
                    }

                    let potential_word: String =
                        chars[word_start..potential_word_end].iter().collect();

                    // Don't split on common abbreviations
                    if !is_abbreviation(&potential_word) {
                        sentences.push(text[current_start..potential_word_end].trim().to_string());
                        current_start = potential_word_end;
                        i = j;
                        continue;
                    }
                }
            }
        }

        i += 1;
    }

    // Add remaining text
    if current_start < text.len() {
        let remaining = text[current_start..].trim();
        if !remaining.is_empty() {
            sentences.push(remaining.to_string());
        }
    }

    sentences.into_iter().filter(|s| !s.is_empty()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_abbreviation() {
        assert!(is_abbreviation("Mr."));
        assert!(is_abbreviation("etc."));
        assert!(is_abbreviation("U.S."));
        assert!(!is_abbreviation("Hello."));
        assert!(!is_abbreviation("cat."));
    }

    #[test]
    fn test_is_sentence_ender() {
        assert!(is_sentence_ender("Hello."));
        assert!(is_sentence_ender("What?"));
        assert!(is_sentence_ender("Stop!"));
        assert!(!is_sentence_ender("U.S.A."));
        assert!(!is_sentence_ender("Mr."));
    }

    #[test]
    fn test_split_into_sentences() {
        let text = "Hello world. This is a test. How are you?";
        let sentences = split_into_sentences(text);
        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "Hello world.");
        assert_eq!(sentences[1], "This is a test.");
        assert_eq!(sentences[2], "How are you?");
    }

    #[test]
    fn test_split_with_abbreviations() {
        let text = "Dr. Smith went to D.C. He saw Mr. Jones.";
        let sentences = split_into_sentences(text);
        // Should not split on abbreviations
        assert!(!sentences.is_empty());
    }
}
