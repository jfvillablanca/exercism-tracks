use std::iter;

pub fn translate(input: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let consonants: Vec<char> =
        ('b'..'z').filter(|c| !vowels.contains(c)).collect();

    let latinize_word = |f: &[char], s: &[char], rest: &[char]| {
        s.iter()
            .map(|&c| c.to_string())
            .chain(rest.iter().map(|&c| c.to_string()))
            .chain(f.iter().map(|&c| c.to_string()))
            .chain(iter::once("ay".to_string()))
            .collect()
    };

    let process_word = |word: &str| -> String {
        match &word.chars().collect::<Vec<char>>()[..] {
            [f, s, 'y', rest @ ..] if consonants.contains(f) => {
                latinize_word(&[*f, *s], &['y'], rest)
            }
            [f, 'q', 'u', rest @ ..] if consonants.contains(f) => {
                latinize_word(&[], rest, &[*f, 'q', 'u'])
            }
            ['q', 'u', rest @ ..] => latinize_word(&[], rest, &['q', 'u']),
            ['y', 't', rest @ ..] => latinize_word(&[], &['y', 't'], rest),
            ['t', 'h', 'r', rest @ ..] => {
                latinize_word(&[], rest, &['t', 'h', 'r'])
            }
            ['s', 'c', 'h', rest @ ..] => {
                latinize_word(&[], rest, &['s', 'c', 'h'])
            }
            ['t', 'h', rest @ ..] => latinize_word(&[], rest, &['t', 'h']),
            ['c', 'h', rest @ ..] => latinize_word(&[], rest, &['c', 'h']),
            ['x', 'r', rest @ ..] => latinize_word(&[], &['x', 'r'], rest),
            [f, s, rest @ ..] if consonants.contains(f) => {
                latinize_word(&[*f], &[*s], rest)
            }
            _ => word.to_string() + "ay",
        }
    };

    input
        .split_whitespace()
        .map(process_word)
        .collect::<Vec<_>>()
        .join(" ")
}
