use crate::Request;

use serde::{Deserialize, Serialize};


/// 批量加密
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DataList {
    
    /// 明文数据
    #[serde(rename = "data")]
    pub data: Option<String>,
    
    /// 是否支持搜索
    #[serde(rename = "search")]
    pub search: Option<bool>,
    
    /// 敏感信息类型. id: 身份证号, phone: 手机号码, simple: 昵称, 地址等
    #[serde(rename = "type")]
    pub type_: Option<String>,
    
}

/// 批量加密
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOpenKmsEncryptBatch {
    
    /// 要加密的数据列表, 列表大小不超过100
    #[serde(rename = "data_list")]
    pub data_list: Option<Vec<DataList>>,
    
}


impl Request for PddOpenKmsEncryptBatch {
    fn get_type() -> String {
        "pdd.open.kms.encrypt.batch".to_string()
    }

    fn get_response_name() -> String {
        "open_kms_encrypt_batch_response".to_string()
    }
}
