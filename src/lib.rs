#[macro_use]
extern crate failure;

pub mod rpc;
mod types;
mod utils;

pub use rpc::OKChainRpcClient;
