use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询云打印机状态
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CloudPrinterStatusQueryRequest {
    
    /// 打印机id
    #[serde(rename = "printer_id")]
    pub printer_id: Option<String>,
    
    /// 共享码
    #[serde(rename = "share_code")]
    pub share_code: Option<String>,
    
}

/// 查询云打印机状态
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddCloudPrinterStatusQuery {
    
    /// 打印状态查询请求
    #[serde(rename = "cloud_printer_status_query_request")]
    pub cloud_printer_status_query_request: Option<CloudPrinterStatusQueryRequest>,
    
}


/// 查询云打印机状态
impl Request for PddCloudPrinterStatusQuery {
    fn get_type() -> String {
        "pdd.cloud.printer.status.query".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
