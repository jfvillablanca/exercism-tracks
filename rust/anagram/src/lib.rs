use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalized_word = word.to_lowercase();
    let mut anagram_set: HashSet<&str> = HashSet::new();

    for &candidate in possible_anagrams {
        let normalized_candidate = candidate.to_lowercase();
        if candidate.len() != word.len() {
            continue;
        }
        if normalized_candidate == normalized_word {
            continue;
        }

        let mut word_graphemes: Vec<&str> = normalized_word.graphemes(true).collect();
        let candidate_graphemes: Vec<&str> = normalized_candidate.graphemes(true).collect();

        for gr in candidate_graphemes {
            if word_graphemes.contains(&gr) {
                let p = word_graphemes.iter().position(|&x| x == gr).unwrap();
                word_graphemes.swap_remove(p);
            }
        }

        if word_graphemes.is_empty() {
            anagram_set.insert(candidate);
        }
    }

    anagram_set
}
