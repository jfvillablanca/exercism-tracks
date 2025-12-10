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
                (0, 0) if significand.len() > moving_scale => {
                    significand.pop_back()
                }
                (0, x) if x != 0 && significand.len() > 2 => {
                    significand.pop_back()
                }
                (0, _) if moving_scale == 1 => significand.pop_back(),
                _ => {
                    break;
                }
            };
        }

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

    fn denormalize_for_operation(
        lhs: &Decimal,
        rhs: &Decimal,
        is_msb: bool,
    ) -> (Vec<u8>, Vec<u8>) {
        let max_frac = lhs.scale.max(rhs.scale);
        let min_frac = lhs.scale.min(rhs.scale);
        let max_int = (lhs.significand.len() - lhs.scale)
            .max(rhs.significand.len() - rhs.scale);
        let min_int = (lhs.significand.len() - lhs.scale)
            .min(rhs.significand.len() - rhs.scale);

        let right_pad = vec![0; max_frac - min_frac];
        let left_pad = vec![0; max_int - min_int];

        let lhs_rev: Vec<u8> = lhs.significand.iter().copied().rev().collect();
        let rhs_rev: Vec<u8> = rhs.significand.iter().copied().rev().collect();

        let apply_rev = |v: Vec<u8>| {
            if is_msb {
                v
            } else {
                v.into_iter().rev().collect()
            }
        };

        match lhs.scale.cmp(&rhs.scale) {
            Ordering::Greater => {
                let lhs_out = [left_pad.clone(), lhs_rev.clone()].concat();
                let rhs_out = [rhs_rev.clone(), right_pad.clone()].concat();
                (apply_rev(lhs_out), apply_rev(rhs_out))
            }
            Ordering::Less => {
                let lhs_out = [lhs_rev.clone(), right_pad.clone()].concat();
                let rhs_out = [left_pad.clone(), rhs_rev.clone()].concat();
                (apply_rev(lhs_out), apply_rev(rhs_out))
            }
            Ordering::Equal => (apply_rev(lhs_rev), apply_rev(rhs_rev)),
        }
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
        ) = Decimal::denormalize_for_operation(self, other, true);
        match (
            self_denormalized_significand_msb
                .cmp(&other_denormalized_significand_msb),
            self.is_negative,
        ) {
            (Ordering::Greater, true) => Some(Ordering::Less),
            (Ordering::Greater, false) => Some(Ordering::Greater),
            (Ordering::Less, true) => Some(Ordering::Greater),
            (Ordering::Less, false) => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
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
