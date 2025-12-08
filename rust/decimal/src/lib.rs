use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use itertools::Itertools;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq)]
pub struct Decimal {
    /// Least-significant-limb
    significand: Vec<u8>,
    /// Canonical form, minimum scale of 1
    scale: usize,

    /// Sign bit
    is_negative: bool,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if input.is_empty() {
            return None;
        };

        let (significand, is_negative) =
            Decimal::extract_significand_and_sign(input)?;
        let scale = Decimal::determine_scale(input);

        let decimal = Decimal::normalize(significand, scale, is_negative);

        Some(decimal)
    }

    fn normalize(
        significand: Vec<u8>,
        scale: usize,
        is_negative: bool,
    ) -> Decimal {
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
        let rev_significand_iter =
            significand.clone().into_iter().rev().tuple_windows();

        for (leading, next_leading) in rev_significand_iter {
            match (leading, next_leading) {
                (0, 0) if { significand.len() > 2 } => significand.pop_back(),
                (0, _) if { significand.len() > 2 } => significand.pop_back(),
                _ => {
                    break;
                }
            };
        }

        // Invariant on canonical form
        // Example: zero is Decimal { [0, 0], 1, false }
        assert!(significand.len() >= 2);
        assert!(moving_scale >= 1);

        Decimal {
            significand: significand.into_iter().collect(),
            scale: moving_scale,
            is_negative,
        }
    }

    fn extract_significand_and_sign(input: &str) -> Option<(Vec<u8>, bool)> {
        // very rudimentary check
        let is_negative = input.contains('-');
        let input_iter = input.chars().rev();
        let mut significand = VecDeque::new();

        for c in input_iter {
            match c {
                '0'..='9' => {
                    significand.push_back(c.to_digit(10).unwrap() as u8)
                }
                '.' | '-' | '+' => {}
                _ => return None,
            }
        }
        // The canonical form includes a single trailing zero after the point
        // Since Decimal stores least-significant-limb first, we push the
        // extra zero to the front
        if !input.contains('.') {
            significand.push_front(0);
        }

        Some((significand.into(), is_negative))
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
        write!(f, "{}{}", if self.is_negative { "-" } else { "" }, display)
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let sign_compare = self.is_negative.cmp(&other.is_negative);
        if sign_compare.eq(&Ordering::Greater) {
            return Some(Ordering::Less);
        } else if sign_compare.eq(&Ordering::Less) {
            return Some(Ordering::Greater);
        }

        let (
            self_denormalized_significand_msb,
            other_denormalized_significand_msb,
        ): (Vec<u8>, Vec<u8>) = match self.scale.cmp(&other.scale) {
            Ordering::Greater => (
                self.significand
                    .clone()
                    .into_iter()
                    .chain(vec![0; self.scale.sub(other.scale)])
                    .rev()
                    .collect(),
                vec![0; self.scale.sub(other.scale)]
                    .iter()
                    .rev()
                    .chain(&other.significand)
                    .copied()
                    .collect(),
            ),
            Ordering::Less => (
                vec![0; other.scale.sub(self.scale)]
                    .iter()
                    .rev()
                    .chain(&self.significand)
                    .copied()
                    .collect(),
                other.significand
                    .clone()
                    .into_iter()
                    .chain(vec![0; other.scale.sub(self.scale)])
                    .rev()
                    .collect(),
            ),
            _ => (
                self.significand.clone().into_iter().rev().collect(),
                other.significand.clone().into_iter().rev().collect(),
            ),
        };

        self_denormalized_significand_msb
            .partial_cmp(&other_denormalized_significand_msb)
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
