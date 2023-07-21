use crate::Request;

use serde::{Deserialize, Serialize};


/// 商家换货发货
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddRefundExchangeShippingRequest {
    
    /// 售后id
    #[serde(rename = "after_sales_id")]
    pub after_sales_id: Option<i64>,
    
    /// 订单编号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 物流公司id
    #[serde(rename = "shipping_id")]
    pub shipping_id: Option<i32>,
    
    /// 物流公司名称
    #[serde(rename = "shipping_name")]
    pub shipping_name: Option<String>,
    
    /// 换货物流单号
    #[serde(rename = "tracking_number")]
    pub tracking_number: Option<String>,
    
}

/// 商家换货发货
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddRefundExchangeShipping {
    
    /// request
    #[serde(rename = "request")]
    pub request: Option<PddRefundExchangeShippingRequest>,
    
}


impl Request for PddRefundExchangeShipping {
    fn get_type() -> String {
        "pdd.refund.exchange.shipping".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
