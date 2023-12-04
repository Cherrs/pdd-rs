use crate::Request;

use serde::{Deserialize, Serialize};


/// 三方快递服务商回传寄件单的内网重量
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddServiceMailCheckWeightInfoRequest {
    
    /// 物流环节首发或到达省中心称重，单位克
    #[serde(rename = "centerWeight")]
    pub center_weight: Option<i32>,
    
    /// 结算重量，单位克
    #[serde(rename = "checkWeight")]
    pub check_weight: Option<i32>,
    
    /// 快递公司编码
    #[serde(rename = "expressCode")]
    pub express_code: Option<String>,
    
    /// 是否使用抛重
    #[serde(rename = "isUseVolumetric")]
    pub is_use_volumetric: Option<bool>,
    
    /// 运单号
    #[serde(rename = "mailNo")]
    pub mail_no: Option<String>,
    
    /// 抛重体积
    #[serde(rename = "volume")]
    pub volume: Option<i32>,
    
}

/// 三方快递服务商回传寄件单的内网重量
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddServiceMailCheckWeightInfo {
    
    /// 请求参数
    #[serde(rename = "request")]
    pub request: Option<PddServiceMailCheckWeightInfoRequest>,
    
}


/// 三方快递服务商回传寄件单的内网重量
impl Request for PddServiceMailCheckWeightInfo {
    fn get_type() -> String {
        "pdd.service.mail.check.weight.info".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
