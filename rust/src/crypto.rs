use crate::config::P;
use num_bigint::BigUint;
use num_traits::{One, Zero};

pub fn mod_exp(mut base: BigUint, mut exp: BigUint) -> BigUint {
    let mut result = BigUint::one();
    base = &base % &*P;
    while exp > BigUint::zero() {
        if &exp % 2u32 == BigUint::one() {
            result = (&result * &base) % &*P;
        }
        exp >>= 1;
        base = (&base * &base) % &*P;
    }
    result
}
