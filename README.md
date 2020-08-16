<h1 align="center">okchain-rust-sdk</h1>
<p align="center">
    Rust SDK for OKChain
</p>

[![travis-ci.org](https://travis-ci.org/zjhmale/okchain-rust-sdk.svg)](https://travis-ci.org/zjhmale/okchain-rust-sdk)

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
okchain-rust-sdk = { git = "https://github.com/zjhmale/okchain-rust-sdk", branch = "master" }
```

Then check out the examples below.

# Examples

See [examples](examples) for runnable examples.

## Quick start


```rust
use okchain_sdk_lib::*;

pub fn main() {
    let rpc_client = OKChainRpcClient::new("http://127.0.0.1:26657");
    let result = rpc_client.get_token("okb");
    println!("{:?}", result);
}
```

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

