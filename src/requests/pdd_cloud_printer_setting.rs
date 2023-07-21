use crate::Request;

use serde::{Deserialize, Serialize};


/// 设置云打印机
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddCloudPrinterSetting {
    
    /// 开平请求基类
    #[serde(rename = "request")]
    pub request: Option<PddCloudPrinterSettingRequest>,
    
}

/// 设置云打印机
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddCloudPrinterSettingRequest {
    
    /// 打印浓度（1-淡，2-正常，3-浓）
    #[serde(rename = "density")]
    pub density: Option<i32>,
    
    /// 打印机标识
    #[serde(rename = "printer_id")]
    pub printer_id: Option<String>,
    
    /// 共享码
    #[serde(rename = "share_code")]
    pub share_code: Option<String>,
    
}


impl Request for PddCloudPrinterSetting {
    fn get_type() -> String {
        "pdd.cloud.printer.setting".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
