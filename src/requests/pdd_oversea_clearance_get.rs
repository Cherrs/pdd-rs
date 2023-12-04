use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取多多国际清关材料（按订单维度获取）
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOverseaClearanceGet {
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
}


/// 获取多多国际清关材料（按订单维度获取）
impl Request for PddOverseaClearanceGet {
    fn get_type() -> String {
        "pdd.oversea.clearance.get".to_string()
    }

    fn get_response_name() -> String {
        "clearance_response".to_string()
    }
}
