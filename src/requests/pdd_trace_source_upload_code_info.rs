use crate::Request;

use serde::{Deserialize, Serialize};


/// 溯源服务商上传溯源码信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTraceSourceUploadCodeInfo {
    
    /// 溯源码列表
    #[serde(rename = "serial_num_list")]
    pub serial_num_list: Option<Vec<SerialNumList>>,
    
}

/// 溯源服务商上传溯源码信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SerialNumList {
    
    /// 溯源码（处理后）
    #[serde(rename = "encoded_serial_num")]
    pub encoded_serial_num: Option<String>,
    
    /// 溯源码
    #[serde(rename = "serial_num")]
    pub serial_num: Option<String>,
    
}


/// 溯源服务商上传溯源码信息
impl Request for PddTraceSourceUploadCodeInfo {
    fn get_type() -> String {
        "pdd.trace.source.upload.code.info".to_string()
    }

    fn get_response_name() -> String {
        "status".to_string()
    }
}
