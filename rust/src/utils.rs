use num_bigint::BigUint;
use num_traits::{One, ToPrimitive};

pub fn combine_u256(limb0: u64, limb1: u64, limb2: u64, limb3: u64) -> BigUint {
    let limb0 = BigUint::from(limb0);
    let limb1 = BigUint::from(limb1) << 64; // Use bitwise shift for powers of 2
    let limb2 = BigUint::from(limb2) << 128;
    let limb3 = BigUint::from(limb3) << 192;

    limb0 + limb1 + limb2 + limb3
}

pub fn split_u256(value: BigUint) -> (u64, u64, u64, u64) {
    let mask: BigUint = (BigUint::one() << 64) - BigUint::one();

    let limb0 = (&value & &mask).to_u64().expect("Conversion to u64 failed");
    let limb1 = ((&value >> 64u32) & &mask)
        .to_u64()
        .expect("Conversion to u64 failed");
    let limb2 = ((&value >> 128u32) & &mask)
        .to_u64()
        .expect("Conversion to u64 failed");
    let limb3 = ((&value >> 192u32) & &mask)
        .to_u64()
        .expect("Conversion to u64 failed");

    (limb0, limb1, limb2, limb3)
}
