use crate::Request;

use serde::{Deserialize, Serialize};


/// 云打印机绑定
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CloudPrinterBindRequest {
    
    /// 打印机id
    #[serde(rename = "printer_id")]
    pub printer_id: Option<String>,
    
    /// 验证码
    #[serde(rename = "verify_code")]
    pub verify_code: Option<String>,
    
}

/// 云打印机绑定
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddCloudPrinterBind {
    
    /// 云打印绑定请求
    #[serde(rename = "cloud_printer_bind_request")]
    pub cloud_printer_bind_request: Option<CloudPrinterBindRequest>,
    
}


impl Request for PddCloudPrinterBind {
    fn get_type() -> String {
        "pdd.cloud.printer.bind".to_string()
    }

    fn get_response_name() -> String {
        "cloud_printer_bind_response".to_string()
    }
}
