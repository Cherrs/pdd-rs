use crate::Request;

use serde::{Deserialize, Serialize};


/// erp打单信息同步
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddErpOrderSync {
    
    /// 物流公司编码
    #[serde(rename = "logistics_id")]
    pub logistics_id: Option<i64>,
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 订单状态：1-已打单
    #[serde(rename = "order_state")]
    pub order_state: Option<i32>,
    
    /// 运单号
    #[serde(rename = "waybill_no")]
    pub waybill_no: Option<String>,
    
}


impl Request for PddErpOrderSync {
    fn get_type() -> String {
        "pdd.erp.order.sync".to_string()
    }

    fn get_response_name() -> String {
        "error_code".to_string()
    }
}
