pub fn find(xs: &[i32], key: i32) -> Option<usize> {
    find_(xs, key, 0, xs.len())
}

fn find_(xs: &[i32], key: i32, lo: usize, hi: usize) -> Option<usize> {
    let len = hi - lo;
    if len == 0 {
        return None;
    }
    let mid = (hi + lo) / 2;

    if len.is_multiple_of(2) {
        match key.cmp(&xs[mid]) {
            std::cmp::Ordering::Less => find_(xs, key, lo, mid),
            _ => find_(xs, key, mid, hi),
        };
    }

    match key.cmp(&xs[mid]) {
        std::cmp::Ordering::Equal => Some(mid),
        std::cmp::Ordering::Less => find_(xs, key, lo, mid),
        std::cmp::Ordering::Greater => find_(xs, key, mid + 1, hi),
    }
}
