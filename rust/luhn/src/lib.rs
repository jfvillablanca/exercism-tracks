/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let nums = parse_digits(code);

    if nums.iter().eq(vec![&0]) || nums.is_empty() {
        return false;
    }

    nums.iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| {
            if (i + 1).is_multiple_of(2) {
                let double = x * 2;
                if double > 9 {
                    return double - 9;
                };
                return double;
            }
            x
        })
        .sum::<u32>()
        .is_multiple_of(10)
}

fn parse_digits(code: &str) -> Vec<u32> {
    let mut v = Vec::new();

    for c in code.chars() {
        match c {
            '0'..='9' => v.push(c.to_digit(10).unwrap()),
            c if c.is_ascii_whitespace() => {}
            _ => return Vec::new()
        }
    }
    v
}
