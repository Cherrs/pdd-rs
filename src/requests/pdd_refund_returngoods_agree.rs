use crate::Request;

use serde::{Deserialize, Serialize};


/// 开放平台商家同意退货
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddRefundReturngoodsAgreeRequest {
    
    /// 售后id
    #[serde(rename = "after_sales_id")]
    pub after_sales_id: Option<i64>,
    
    /// 给用户留言
    #[serde(rename = "operate_desc")]
    pub operate_desc: Option<String>,
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 退货地址列表中已有的退货地址id
    #[serde(rename = "return_address_id")]
    pub return_address_id: Option<String>,
    
}

/// 开放平台商家同意退货
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddRefundReturngoodsAgree {
    
    /// 请求入参
    #[serde(rename = "request")]
    pub request: Option<PddRefundReturngoodsAgreeRequest>,
    
}


impl Request for PddRefundReturngoodsAgree {
    fn get_type() -> String {
        "pdd.refund.returngoods.agree".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
