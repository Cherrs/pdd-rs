use serde::{Deserialize, Serialize};

/// 公共请求参数
#[derive(Serialize, Deserialize, Debug)]
pub struct PublicParameters {
    #[serde(rename = "type")]
    pub type_: String,
    pub client_id: String,
    pub timestamp: i64,
    pub data_type: Option<String>,
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
