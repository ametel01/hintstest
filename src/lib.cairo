use core::traits::Into;
mod oracle;
#[cfg(test)]
mod tests {
    mod call_test;
}

use oracle::{Request, Response, ModExpOracle, request::Base, request::Exp};

const BASE: u256 = 18446744073709551616;

#[derive(Copy, Debug, Drop)]
struct U256 {
    limb0: u64,
    limb1: u64,
    limb2: u64,
    limb3: u64,
}

fn main() {
    let base: u256 = 88009966869426586326054793412191768538473160230674771117477940242756393061596;
    let exp: u256 = 26859873616407055777888130878028564851092662860690821559568496789771229719829;

    let base_split = split_u256(base);
    let base = Base {
        limb0: base_split.limb0,
        limb1: base_split.limb1,
        limb2: base_split.limb2,
        limb3: base_split.limb3
    };
    let exp_split = split_u256(exp);
    let exp = Exp {
        limb0: exp_split.limb0,
        limb1: exp_split.limb1,
        limb2: exp_split.limb2,
        limb3: exp_split.limb3
    };
    let request = Request { base: Option::Some(base), exp: Option::Some(exp) };
    let result = ModExpOracle::mod_exp(request);
    let result = reconstruct_u256(
        U256 { limb0: result.limb0, limb1: result.limb1, limb2: result.limb2, limb3: result.limb3 }
    );

    println!("Result: {}", result);
}

fn split_u256(x: u256) -> U256 {
    let limb0: u64 = (x % BASE).try_into().unwrap();
    let limb1: u64 = ((x / BASE) % BASE).try_into().unwrap();
    let limb2: u64 = ((x / (BASE * BASE)) % BASE).try_into().unwrap();
    let limb3: u64 = ((x / (BASE * BASE * BASE)) % BASE).try_into().unwrap();
    U256 { limb0, limb1, limb2, limb3 }
}

fn reconstruct_u256(limbs: U256) -> u256 {
    let x = limbs.limb0.into()
        + limbs.limb1.into() * BASE
        + limbs.limb2.into() * (BASE * BASE)
        + limbs.limb3.into() * (BASE * BASE * BASE);
    x
}

