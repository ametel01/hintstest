#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<request::Base>,
    #[prost(message, optional, tag = "2")]
    pub exp: ::core::option::Option<request::Exp>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Base {
        #[prost(uint64, tag = "1")]
        pub limb0: u64,
        #[prost(uint64, tag = "2")]
        pub limb1: u64,
        #[prost(uint64, tag = "3")]
        pub limb2: u64,
        #[prost(uint64, tag = "4")]
        pub limb3: u64,
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Exp {
        #[prost(uint64, tag = "1")]
        pub limb0: u64,
        #[prost(uint64, tag = "2")]
        pub limb1: u64,
        #[prost(uint64, tag = "3")]
        pub limb2: u64,
        #[prost(uint64, tag = "4")]
        pub limb3: u64,
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(uint64, tag = "1")]
    pub limb0: u64,
    #[prost(uint64, tag = "2")]
    pub limb1: u64,
    #[prost(uint64, tag = "3")]
    pub limb2: u64,
    #[prost(uint64, tag = "4")]
    pub limb3: u64,
}
