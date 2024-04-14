use starknet::testing::cheatcode;
#[derive(Copy, Debug, Drop, Serde)]
struct Request {
    limb0: u64,
    limb1: u64,
    limb2: u64,
    limb3: u64,
}
#[derive(Copy, Debug, Drop, Serde)]
struct Response {
    limb0: u64,
    limb1: u64,
    limb2: u64,
    limb3: u64,
}
#[generate_trait]
impl SqrtOracle of SqrtOracleTrait {
    fn sqrt(arg: super::oracle::Request) -> super::oracle::Response {
        let mut serialized = ArrayTrait::new();
        arg.serialize(ref serialized);
        let mut result = cheatcode::<'sqrt'>(serialized.span());
        Serde::deserialize(ref result).unwrap()
    }
}
