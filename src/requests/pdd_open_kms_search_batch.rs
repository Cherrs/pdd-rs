use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取搜索索引
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOpenKmsSearchBatch {
    
    /// 数据列表, 列表大小不超过100
    #[serde(rename = "input_list")]
    pub input_list: Option<Vec<InputList>>,
    
}

/// 获取搜索索引
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InputList {
    
    /// 搜索内容
    #[serde(rename = "input")]
    pub input: Option<String>,
    
    /// 敏感信息类型. id: 身份证号, phone: 手机号码, simple: 昵称, 地址等
    #[serde(rename = "type")]
    pub type_: Option<String>,
    
}


/// 获取搜索索引
impl Request for PddOpenKmsSearchBatch {
    fn get_type() -> String {
        "pdd.open.kms.search.batch".to_string()
    }

    fn get_response_name() -> String {
        "open_kms_search_batch_response".to_string()
    }
}
