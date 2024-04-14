use core::traits::Into;
mod oracle;
#[cfg(test)]
mod tests {
    mod call_test;
}

use oracle::{Request, Response, SqrtOracle};

const BASE: u256 = 18446744073709551616;

fn main() -> bool {
    let num: u256 = 115792089237316195423570985008687907852589419931798687112530834793049593217025;

    let request = split_u256(num);
    let result = SqrtOracle::sqrt(request);
    let result = reconstruct_u256(result);

    result * result == num
}

fn split_u256(x: u256) -> Request {
    let limb0: u64 = (x % BASE).try_into().unwrap();
    let limb1: u64 = ((x / BASE) % BASE).try_into().unwrap();
    let limb2: u64 = ((x / (BASE * BASE)) % BASE).try_into().unwrap();
    let limb3: u64 = ((x / (BASE * BASE * BASE)) % BASE).try_into().unwrap();
    Request { limb0, limb1, limb2, limb3 }
}

fn reconstruct_u256(limbs: Response) -> u256 {
    let x = limbs.limb0.into()
        + limbs.limb1.into() * BASE
        + limbs.limb2.into() * (BASE * BASE)
        + limbs.limb3.into() * (BASE * BASE * BASE);
    x
}

