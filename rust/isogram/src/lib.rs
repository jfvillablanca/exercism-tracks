use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut char_set = HashSet::new();
    let candidate_chars: Vec<char> = candidate.chars().map(|c| c.to_ascii_lowercase()).collect();
    for ch in candidate_chars.iter() {
        if ch.is_alphabetic() {
            if char_set.contains(&ch) {
                return false;
            }
            char_set.insert(ch);
        }
    }
    true
}
