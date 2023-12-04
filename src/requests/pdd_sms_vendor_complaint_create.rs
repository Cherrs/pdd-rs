use crate::Request;

use serde::{Deserialize, Serialize};


/// 短信供应商投诉号码上传
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddSmsVendorComplaintCreate {
    
    /// cmpp账号名
    #[serde(rename = "account")]
    pub account: Option<String>,
    
    /// 投诉时间(格式yyyy-MM-dd HH:mm:ss)
    #[serde(rename = "complaint_time")]
    pub complaint_time: Option<String>,
    
    /// 投诉次数
    #[serde(rename = "count")]
    pub count: Option<i32>,
    
    /// 短信下发时间(格式yyyy-MM-dd HH:mm:ss)
    #[serde(rename = "deliver_time")]
    pub deliver_time: Option<String>,
    
    /// 手机号码
    #[serde(rename = "mobile")]
    pub mobile: Option<String>,
    
    /// 归属运营商
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    
    /// 归属地省份
    #[serde(rename = "province")]
    pub province: Option<String>,
    
    /// 短信投诉内容(不超过500个字)
    #[serde(rename = "sms_content")]
    pub sms_content: Option<String>,
    
}


/// 短信供应商投诉号码上传
impl Request for PddSmsVendorComplaintCreate {
    fn get_type() -> String {
        "pdd.sms.vendor.complaint.create".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
