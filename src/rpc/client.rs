use crate::types::*;
use crate::utils::*;
use hex;
use reqwest::Response;
use serde_json::Value;

pub struct OKChainRpcClient {
    client: reqwest::Client,
    url: reqwest::Url,
}

impl OKChainRpcClient {
    pub fn new(uri: &str) -> Self {
        let url =
            reqwest::Url::parse(uri).expect("okchain rpc uri, e.g. \"http://127.0.0.1:26657\"");
        OKChainRpcClient {
            url: url,
            client: reqwest::Client::new(),
        }
    }

    pub fn query_to_bm(&self, resp: &mut Response) -> Result<BaseModel, failure::Error> {
        // For debug purpose
        // let output = resp.json::<HashMap<String, Value>>()?;
        // println!("{}", serde_json::to_string_pretty(&output).unwrap());

        let result = resp.json::<OKChainRpcResult>()?;
        Ok(BaseModel::from(result))
    }

    // Account

    // Transaction

    // Query
    pub fn abci_query(
        &self,
        path: &str,
        data: Option<Vec<u8>>,
    ) -> Result<Response, failure::Error> {
        let mut req_json = serde_json::json!({
            "id": "jsonrpc-client",
            "jsonrpc": "2.0",
            "method": "abci_query",
            "params": {
                "height": "0",
                "path": path,
                "prove": false
            }
        });
        if let Some(payload) = data {
            req_json["params"]["data"] = Value::String(hex::encode(payload));
        }
        let resp = self.client.post(self.url.clone()).json(&req_json).send()?;
        Ok(resp)
    }

    pub fn get_tokens(&self) -> Result<BaseModel, failure::Error> {
        let mut resp = self.abci_query("custom/token/tokens", None)?;
        self.query_to_bm(&mut resp)
    }

    pub fn get_token(&self, symbol: &str) -> Result<BaseModel, failure::Error> {
        let mut resp = self.abci_query(format!("custom/token/info/{}", symbol).as_str(), None)?;
        self.query_to_bm(&mut resp)
    }

    pub fn get_products(&self) -> Result<BaseModel, failure::Error> {
        let mut resp = self.abci_query("custom/token/products", None)?;
        self.query_to_bm(&mut resp)
    }

    pub fn get_depthbook(&self, product: &str) -> Result<BaseModel, failure::Error> {
        ensure!(is_product(product), "invalid product");
        let data = serde_json::json!({
            "Product": product,
            "Size": "200",
        })
        .to_string()
        .into_bytes();

        let mut resp = self.abci_query("custom/token/products", Some(data))?;
        self.query_to_bm(&mut resp)
    }

    pub fn get_candles(
        &self,
        granularity: u32,
        product: &str,
        size: u32,
    ) -> Result<BaseModel, failure::Error> {
        ensure!(is_product(product), "invalid product");
        let data = serde_json::json!({
            "Product": product,
            "Granularity": format!("{}", granularity).as_str(),
            "Size": format!("{}", size).as_str(),
        })
        .to_string()
        .into_bytes();

        let mut resp = self.abci_query("custom/backend/candles", Some(data))?;
        self.query_to_bm(&mut resp)
    }

    pub fn get_tickers(&self, count: u32) -> Result<BaseModel, failure::Error> {
        let data = serde_json::json!({
            "Product": "",
            "Count": format!("{}", count).as_str(),
            "Sort": true,
        })
        .to_string()
        .into_bytes();

        let mut resp = self.abci_query("custom/backend/tickers", Some(data))?;
        self.query_to_bm(&mut resp)
    }
}
