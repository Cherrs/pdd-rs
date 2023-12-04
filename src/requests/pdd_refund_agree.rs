use crate::Request;

use serde::{Deserialize, Serialize};


/// 商家同意退款
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddRefundAgreeRequest {
    
    /// 售后id
    #[serde(rename = "after_sales_id")]
    pub after_sales_id: Option<i64>,
    
    /// 退款备注，商家留言
    #[serde(rename = "operate_desc")]
    pub operate_desc: Option<String>,
    
    /// 订单编号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
}

/// 商家同意退款
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddRefundAgree {
    
    /// request
    #[serde(rename = "request")]
    pub request: Option<PddRefundAgreeRequest>,
    
}


/// 商家同意退款
impl Request for PddRefundAgree {
    fn get_type() -> String {
        "pdd.refund.agree".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
