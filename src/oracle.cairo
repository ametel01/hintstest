use starknet::testing::cheatcode;
#[derive(Drop, Serde)]
struct Request {
    base: Option<super::oracle::request::Base>,
    exp: Option<super::oracle::request::Exp>,
}
/// Nested message and enum types in `Request`.
mod request {
    #[derive(Drop, Serde)]
    struct Base {
        limb0: u64,
        limb1: u64,
        limb2: u64,
        limb3: u64,
    }
    #[derive(Drop, Serde)]
    struct Exp {
        limb0: u64,
        limb1: u64,
        limb2: u64,
        limb3: u64,
    }
}
#[derive(Drop, Serde)]
struct Response {
    limb0: u64,
    limb1: u64,
    limb2: u64,
    limb3: u64,
}
#[generate_trait]
impl ModExpOracle of ModExpOracleTrait {
    fn mod_exp(arg: super::oracle::Request) -> super::oracle::Response {
        let mut serialized = ArrayTrait::new();
        arg.serialize(ref serialized);
        let mut result = cheatcode::<'mod_exp'>(serialized.span());
        Serde::deserialize(ref result).unwrap()
    }
}
