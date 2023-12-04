use crate::Request;

use serde::{Deserialize, Serialize};


/// 云打印机绑定时，打印验证码
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CloudPrintVerifyCodeRequest {
    
    /// 打印机id
    #[serde(rename = "printer_id")]
    pub printer_id: Option<String>,
    
}

/// 云打印机绑定时，打印验证码
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddCloudPrintVerifyCode {
    
    /// 云打印验证码请求
    #[serde(rename = "cloud_print_verify_code_request")]
    pub cloud_print_verify_code_request: Option<CloudPrintVerifyCodeRequest>,
    
}


/// 云打印机绑定时，打印验证码
impl Request for PddCloudPrintVerifyCode {
    fn get_type() -> String {
        "pdd.cloud.print.verify.code".to_string()
    }

    fn get_response_name() -> String {
        "cloud_print_verify_code_response".to_string()
    }
}
