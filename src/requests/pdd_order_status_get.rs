use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取订单的状态
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOrderStatusGet {
    
    /// 20150909-452750051,20150909-452750134 用逗号分开
    #[serde(rename = "order_sns")]
    pub order_sns: Option<String>,
    
}


/// 获取订单的状态
impl Request for PddOrderStatusGet {
    fn get_type() -> String {
        "pdd.order.status.get".to_string()
    }

    fn get_response_name() -> String {
        "order_status_get_response".to_string()
    }
}
