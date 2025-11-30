/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let nums: Vec<char> = code
        .chars()
        .filter(|&x| !x.is_ascii_whitespace())
        .collect();

    if nums.len() <= 1 {
        return false;
    }

    let nums = nums.iter().map(|x| x.to_digit(10));

    if nums.clone().any(|x| x.is_none()) {
        return false;
    }

    let nums = nums.flatten().rev().enumerate().map(|(i, x)| {
        if (i + 1).is_multiple_of(2) {
            dbg!(i, x);
            let double = x * 2;
            if double > 9 {
                return double - 9;
            };
            return double;
        }
        x
    });

    nums.sum::<u32>().is_multiple_of(10)
}
