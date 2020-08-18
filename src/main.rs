use okchain_sdk_lib::*;

pub fn main() {
    let rpc_client = OKChainRpcClient::new("tcp://35.176.62.211:26657");
    let result = rpc_client.get_tokens();
    println!("{:?}", result);

    let result = rpc_client.get_token("tokt");
    println!("{:?}", result);
}
