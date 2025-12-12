use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::Display,
    iter::zip,
    ops::{Add, Mul, Sub},
};

use itertools::Itertools;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq)]
pub struct Decimal {
    /// Least-significant-limb
    significand: Vec<u8>,
    /// Canonical form, minimum scale of 0
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
        let mut frac_len = scale;
        let mut significand = VecDeque::from(significand);
        let mut significand_iter = significand.clone().into_iter().peekable();
        while let Some(&trailing) = significand_iter.peek() {
            if trailing.ne(&0) {
                break;
            }
            if trailing.eq(&0) && frac_len >= 1 {
                frac_len = frac_len.saturating_sub(1);
                significand.pop_front();
            }
            significand_iter.next();
        }

        // Remove leading zeros
        let rev_significand_iter =
            significand.clone().into_iter().rev().tuple_windows();

        for (leading, next_leading) in rev_significand_iter {
            match (leading, next_leading) {
                (0, 0) if significand.len() > frac_len => {
                    significand.pop_back()
                }
                (0, x) if x != 0 && significand.len() > frac_len => {
                    significand.pop_back()
                }
                (0, _) if frac_len == 0 => significand.pop_back(),
                _ => {
                    break;
                }
            };
        }

        // If only zero is left, we represent zero as an empty vec
        if significand.len() == 1 && significand.contains(&0) {
            significand.pop_back();
        }

        Decimal {
            significand: significand.into_iter().collect(),
            scale: frac_len,
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
    ) -> (Vec<u8>, Vec<u8>, (usize, usize)) {
        let max_frac = lhs.scale.max(rhs.scale);
        let min_frac = lhs.scale.min(rhs.scale);
        let max_int = (lhs.significand.len() - lhs.scale)
            .max(rhs.significand.len() - rhs.scale);
        let min_int = (lhs.significand.len() - lhs.scale)
            .min(rhs.significand.len() - rhs.scale);

        let right_pad = vec![0; max_frac - min_frac];
        let left_pad = vec![0; max_int - min_int];
        let pad_for_zero_value_decimal = if !left_pad.is_empty() {
            left_pad.clone()
        } else {
            right_pad.clone()
        };

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
                assert_eq!(lhs_out.len(), rhs_out.len());
                (apply_rev(lhs_out), apply_rev(rhs_out), (max_frac, max_int))
            }
            Ordering::Less => {
                let lhs_out = [lhs_rev.clone(), right_pad.clone()].concat();
                let rhs_out = [left_pad.clone(), rhs_rev.clone()].concat();
                assert_eq!(lhs_out.len(), rhs_out.len());
                (apply_rev(lhs_out), apply_rev(rhs_out), (max_frac, max_int))
            }
            Ordering::Equal => {
                // Handles zero-length significands,
                // i.e. zero-valued Decimals compared with integrals
                let lhs_out = if lhs_rev.is_empty() {
                    pad_for_zero_value_decimal.clone()
                } else {
                    lhs_rev
                };
                let rhs_out = if rhs_rev.is_empty() {
                    pad_for_zero_value_decimal
                } else {
                    rhs_rev
                };
                assert_eq!(lhs_out.len(), rhs_out.len());
                (apply_rev(lhs_out), apply_rev(rhs_out), (max_frac, max_int))
            }
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
            _,
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

    fn add(self, rhs: Self) -> Self::Output {
        let (lhs, rhs, (max_frac, _min_int)) =
            Decimal::denormalize_for_operation(&self, &rhs, false);

        let mut carry_over = 0;
        let mut sum = Vec::new();
        for (lh, rh) in zip(lhs, rhs) {
            let x = (lh + rh) % 10;
            sum.push(x + carry_over);
            carry_over = (lh + rh) / 10;
        }
        if carry_over != 0 {
            sum.push(carry_over)
        };

        Decimal::normalize(sum, max_frac, false)
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
