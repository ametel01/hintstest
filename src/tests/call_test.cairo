use hintstest::oracle::{Request, Response, SqrtOracle};
use hintstest::{split_u256, reconstruct_u256};

#[test]
fn sqrt_test() {
    let num: u256 = 115792089237316195423570985008687907852589419931798687112530834793049593217025;

    let request = split_u256(num);
    let response = SqrtOracle::sqrt(request);
    let result = reconstruct_u256(response);

    assert!(result * result == num);
}

#[test]
fn test_helpers() {
    let num: u256 = 115792089237316195423570985008687907852589419931798687112530834793049593217025;
    let a: Request = split_u256(num);
    let c = reconstruct_u256(
        Response { limb0: a.limb0, limb1: a.limb1, limb2: a.limb2, limb3: a.limb3, }
    );
    println!("c: {:?}", c);
}
