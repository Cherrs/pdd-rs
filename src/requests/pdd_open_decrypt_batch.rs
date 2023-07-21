use crate::Request;

use serde::{Deserialize, Serialize};


/// 批量数据解密
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOpenDecryptBatch {
    
    /// 数据列表, 默认列表大小不超过100
    #[serde(rename = "data_list")]
    pub data_list: Option<Vec<DataList>>,
    
}

/// 批量数据解密
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DataList {
    
    /// 解密tag，对于订单数据是订单号
    #[serde(rename = "data_tag")]
    pub data_tag: Option<String>,
    
    /// 密文
    #[serde(rename = "encrypted_data")]
    pub encrypted_data: Option<String>,
    
}


impl Request for PddOpenDecryptBatch {
    fn get_type() -> String {
        "pdd.open.decrypt.batch".to_string()
    }

    fn get_response_name() -> String {
        "open_decrypt_batch_response".to_string()
    }
}
