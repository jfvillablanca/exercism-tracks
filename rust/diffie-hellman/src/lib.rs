use rand::{thread_rng, Rng};

// NOTE:
// based on right-to-left binary method
// https://en.wikipedia.org/wiki/Modular_exponentiation
fn modular_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0
    }
    let mut result = 1;
    let mut base = base as u128 % modulus as u128;
    let mut exp = exp as u128;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus as u128
        }
        exp >>= 1;
        base = (base * base) % modulus as u128
    }

    result as u64
}

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}
