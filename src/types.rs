use base64;
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Clone, Debug)]
pub struct OKChainRpcResult {
    pub code: u32,
    pub codespace: String,
    pub height: String,
    pub index: String,
    pub info: String,
    pub key: Option<String>,
    pub log: String,
    pub proof: Option<String>,
    pub value: Option<String>,
}

impl OKChainRpcResult {
    #[allow(dead_code)]
    pub fn default() -> OKChainRpcResult {
        OKChainRpcResult {
            code: 0,
            codespace: String::from(""),
            height: String::from(""),
            index: String::from(""),
            info: String::from(""),
            key: None,
            log: String::from(""),
            proof: None,
            value: None,
        }
    }
}

#[derive(Deserialize)]
struct RawRpcResponse {
    #[allow(dead_code)]
    id: String,
    result: RawRpcResult,
}

#[derive(Deserialize)]
struct RawRpcResult {
    response: RawRpcInnerResponse,
}

#[derive(Deserialize)]
struct RawRpcInnerResponse {
    pub code: u32,
    pub codespace: String,
    pub height: String,
    pub index: String,
    pub info: String,
    pub key: Option<String>,
    pub log: String,
    pub proof: Option<String>,
    pub value: Option<String>,
}

impl<'de> Deserialize<'de> for OKChainRpcResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = RawRpcResponse::deserialize(deserializer)?;
        let response = helper.result.response;
        Ok(OKChainRpcResult {
            code: response.code,
            codespace: response.codespace,
            height: response.height,
            index: response.index,
            info: response.info,
            key: response.key,
            log: response.log,
            proof: response.proof,
            value: response.value,
        })
    }
}

#[derive(Clone)]
pub struct BaseModel {
    pub code: u32,
    pub data: Option<String>,
    pub detail_msg: Option<String>,
}

impl fmt::Debug for BaseModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BaseModel")
            .field("code", &self.code)
            .field("data", &self.data)
            .field("detail_msg", &self.detail_msg)
            .finish()
    }
}

impl From<OKChainRpcResult> for BaseModel {
    fn from(result: OKChainRpcResult) -> Self {
        match result.code {
            // if the rpc query succeeds
            0 => BaseModel {
                code: 0,
                data: result.value.and_then(|v| {
                    base64::decode(v)
                        .ok()
                        .and_then(|v| String::from_utf8(v).ok())
                }),
                detail_msg: None,
            },
            // if the rpc query fails
            _ => BaseModel {
                code: result.code,
                data: None,
                detail_msg: Some(result.log),
            },
        }
    }
}

impl BaseModel {
    #[allow(dead_code)]
    pub fn default() -> BaseModel {
        BaseModel {
            code: 0,
            data: None,
            detail_msg: None,
        }
    }
}
