use crate::Request;

use serde::{Deserialize, Serialize};


/// 供应商向拼多多回调售后就结果
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTicketOrderRefundNotifycation {
    
    /// 拼多多制票号
    #[serde(rename = "order_no")]
    pub order_no: Option<String>,
    
    /// 退款金额（分） status=2时必传
    #[serde(rename = "refund_amount")]
    pub refund_amount: Option<i64>,
    
    /// 驳回原因 status=3时必传
    #[serde(rename = "reject_reason")]
    pub reject_reason: Option<String>,
    
    /// 退款流水号
    #[serde(rename = "serial_no")]
    pub serial_no: Option<String>,
    
    /// 受理状态。2.已通过 3.已驳回
    #[serde(rename = "status")]
    pub status: Option<i32>,
    
}


/// 供应商向拼多多回调售后就结果
impl Request for PddTicketOrderRefundNotifycation {
    fn get_type() -> String {
        "pdd.ticket.order.refund.notifycation".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
