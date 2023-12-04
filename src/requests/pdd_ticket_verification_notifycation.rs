use crate::Request;

use serde::{Deserialize, Serialize};


/// 供应商向拼多多发送订单核销通知
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTicketVerificationNotifycation {
    
    /// 拼多多制票号
    #[serde(rename = "order_no")]
    pub order_no: Option<String>,
    
    /// 核销时间（13位毫秒数）
    #[serde(rename = "verify_time")]
    pub verify_time: Option<i64>,
    
}


/// 供应商向拼多多发送订单核销通知
impl Request for PddTicketVerificationNotifycation {
    fn get_type() -> String {
        "pdd.ticket.verification.notifycation".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
