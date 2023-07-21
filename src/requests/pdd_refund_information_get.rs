use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询单个售后单详情
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddRefundInformationGet {
    
    /// 售后单id
    #[serde(rename = "after_sales_id")]
    pub after_sales_id: Option<i64>,
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
}


impl Request for PddRefundInformationGet {
    fn get_type() -> String {
        "pdd.refund.information.get".to_string()
    }

    fn get_response_name() -> String {
        "after_sales_reason".to_string()
    }
}
