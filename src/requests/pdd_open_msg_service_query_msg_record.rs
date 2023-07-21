use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询短信发送记录
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOpenMsgServiceQueryMsgRecord {
    
    /// 短信发送流水
    #[serde(rename = "biz_id")]
    pub biz_id: Option<String>,
    
    /// 分页参数,页码
    #[serde(rename = "page_number")]
    pub page_number: Option<i32>,
    
    /// 分页参数，每页数量。最大值50
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 查询手机号码
    #[serde(rename = "phone_number")]
    pub phone_number: Option<String>,
    
    /// 短信发送日期，支持近30天记录查询，格式yyyyMMdd
    #[serde(rename = "send_date")]
    pub send_date: Option<String>,
    
}


impl Request for PddOpenMsgServiceQueryMsgRecord {
    fn get_type() -> String {
        "pdd.open.msg.service.query.msg.record".to_string()
    }

    fn get_response_name() -> String {
        "request_id".to_string()
    }
}
