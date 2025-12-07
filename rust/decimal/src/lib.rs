use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Add, Mul, Sub},
};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Decimal {
    /// Least-significant-limb
    significand: Vec<u8>,
    /// Canonical form, minimum scale of 1
    scale: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if input.is_empty() {
            return None;
        };

        let significand = Decimal::extract_significand_digits(input)?;
        let scale = Decimal::determine_scale(input);

        let decimal = Decimal::normalize(significand, scale);

        Some(decimal)
    }

    fn normalize(significand: Vec<u8>, scale: usize) -> Decimal {
        // Remove trailing zeros
        let mut moving_scale = scale;
        let mut significand = VecDeque::from(significand);
        let mut significand_iter = significand.clone().into_iter().peekable();
        while let Some(&trailing) = significand_iter.peek() {
            if trailing.ne(&0) {
                break;
            }
            if trailing.eq(&0) && moving_scale > 1 {
                moving_scale = moving_scale.saturating_sub(1);
                significand.pop_front();
            }
            significand_iter.next();
        }

        // Remove leading zeros
        let mut rev_significand_iter =
            significand.clone().into_iter().rev().peekable();
        while let Some(&leading) = rev_significand_iter.peek() {
            if leading.ne(&0) {
                break;
            }
            if leading.eq(&0) && significand.len() > 2 {
                significand.pop_back();
            }
            rev_significand_iter.next();
        }

        Decimal {
            significand: significand.into_iter().collect(),
            scale: moving_scale,
        }
    }

    fn extract_significand_digits(input: &str) -> Option<Vec<u8>> {
        let input_iter = input.chars().rev();
        let mut significand = VecDeque::new();

        for c in input_iter {
            match c {
                '0'..='9' => {
                    significand.push_back(c.to_digit(10).unwrap() as u8)
                }
                '.' => {}
                _ => return None,
            }
        }
        // The canonical form includes a single trailing zero after the point
        // Since Decimal stores least-significant-limb first, we push the
        // extra zero to the front
        if !input.contains('.') {
            significand.push_front(0);
        }

        Some(significand.into())
    }

    fn determine_scale(input: &str) -> usize {
        let mut scale = 1_usize;
        for (i, c) in input.chars().rev().enumerate() {
            if c == '.' {
                scale = i;
            }
        }

        scale
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = self
            .significand
            .iter()
            .map(|&d| (d + b'0') as char)
            .collect::<Vec<char>>();

        display.insert(self.scale, '.');

        let display = display.iter().rev().cloned().collect::<String>();
        write!(f, "{}", display)
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}
