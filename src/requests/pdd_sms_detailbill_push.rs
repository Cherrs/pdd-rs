use crate::Request;

use serde::{Deserialize, Serialize};


/// 短信供应商明细回传
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Details {
    
    /// 短信下发时间(yyyy-MM-dd HH:mm:ss)
    #[serde(rename = "deliver_time")]
    pub deliver_time: Option<String>,
    
    /// 回执状态码，发送成功传DELIVRD
    #[serde(rename = "error_code")]
    pub error_code: Option<String>,
    
    /// 短信id,即SubmitResp.msgId,十进制表示
    #[serde(rename = "msg_id")]
    pub msg_id: Option<i64>,
    
    /// 短信提交时间(yyyy-MM-dd HH:mm:ss)
    #[serde(rename = "submit_time")]
    pub submit_time: Option<String>,
    
}

/// 短信供应商明细回传
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddSmsDetailbillPush {
    
    /// cmpp账号名
    #[serde(rename = "account")]
    pub account: Option<String>,
    
    /// 批次版本，每天数据必须属于同一个批次，如果重传可以批次号增加，平台以最大批次号为准。一般情况下，批次号固定数字，只有当某天上传数据错误需要弃用时，使用增加后的新批次号。
    #[serde(rename = "batch_version")]
    pub batch_version: Option<i64>,
    
    /// 数据日期(格式yyyy-MM-dd)
    #[serde(rename = "date")]
    pub date: Option<String>,
    
    /// 短信明细，detail的列表，list最大100
    #[serde(rename = "details")]
    pub details: Option<Vec<Details>>,
    
}


/// 短信供应商明细回传
impl Request for PddSmsDetailbillPush {
    fn get_type() -> String {
        "pdd.sms.detailbill.push".to_string()
    }

    fn get_response_name() -> String {
        "sms_detailbill_push_resposne".to_string()
    }
}
