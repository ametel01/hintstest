use num_bigint::BigUint;
use num_traits::Num;

lazy_static! {
    pub static ref P: BigUint = BigUint::from_str_radix(
        "21888242871839275222246405745257275088696311157297823662689037894645226208583",
        10
    )
    .unwrap();
}
