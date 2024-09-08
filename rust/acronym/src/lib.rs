fn split_words_with_subsplits(split: &str) -> Vec<&str> {
    let mut split_at = split.len();

    let mut split_iter = split.char_indices().peekable();

    while let Some((_, ch)) = split_iter.next() {
        if let Some((i, next_ch)) = split_iter.peek() {
            if ch.is_lowercase() && next_ch.is_uppercase() {
                split_at = *i;
                break;
            }
        }
    }

    let (left, right) = split.split_at(split_at);
    vec![left, right]
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-'])
        .flat_map(|s| split_words_with_subsplits(s))
        .map(|s| s.trim_matches('_'))
        .filter(|s| !s.is_empty())
        .map(|c| c.chars().next())
        .collect::<Option<String>>()
        .unwrap_or_default()
        .to_uppercase()
}
