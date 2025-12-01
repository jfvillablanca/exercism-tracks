pub struct Luhn {
    xs: Vec<usize>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.xs.iter().eq(vec![&0]) || self.xs.is_empty() {
            return false;
        }
        self.xs
            .iter()
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
            .sum::<usize>()
            .is_multiple_of(10)
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self { xs: parse_digits(input.to_string()) }
    }
}

fn parse_digits(code: String) -> Vec<usize> {
    let mut v = Vec::new();

    for c in code.chars() {
        match c {
            '0'..='9' => v.push(c.to_digit(10).unwrap() as usize),
            c if c.is_ascii_whitespace() => {}
            _ => return Vec::new()
        }
    }
    v
}
