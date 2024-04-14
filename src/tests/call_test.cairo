use hintstest::oracle::{Request, Response, ModExpOracle, request::Base, request::Exp};
use hintstest::{split_u256, reconstruct_u256, U256};

#[test]
fn sqrt_test() {
    let base = 88009966869426586326054793412191768538473160230674771117477940242756393061596;
    let exp = 26859873616407055777888130878028564851092662860690821559568496789771229719829;
    let expected = 4957037684828371349028486368725664008193737461084841662327841118011875819652;

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

    assert!(result == expected);
}

