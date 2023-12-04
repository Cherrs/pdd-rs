use crate::Request;

use serde::{Deserialize, Serialize};


/// 用于ISV查询自己名下的服务的月度结算明细
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddServicemarketSettlementbillGet {
    
    /// 分页页码，最大不能超过1000
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 分页大小，最大不能超过100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 服务订单号
    #[serde(rename = "service_order_sn")]
    pub service_order_sn: Option<String>,
    
    /// 结算月份
    #[serde(rename = "settle_month")]
    pub settle_month: Option<String>,
    
}


/// 用于ISV查询自己名下的服务的月度结算明细
impl Request for PddServicemarketSettlementbillGet {
    fn get_type() -> String {
        "pdd.servicemarket.settlementbill.get".to_string()
    }

    fn get_response_name() -> String {
        "settlement_bill_search_response".to_string()
    }
}
