use okchain_sdk_lib::*;

pub fn main() {
    let rpc_client = OKChainRpcClient::new("http://127.0.0.1:26657");
    let result = rpc_client.get_token("okb");
    println!("{:?}", result);
}
