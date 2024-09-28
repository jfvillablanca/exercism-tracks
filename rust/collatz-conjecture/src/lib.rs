pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        n => rec(0, n),
    }
}

fn rec(count: u64, n: u64) -> Option<u64> {
    match n {
        1 => Some(count),
        _ => match n & 1 {
            0 => rec(count + 1, n / 2),
            _ => rec(count + 1, n.checked_mul(3)?.checked_add(1)?),
        },
    }
}
