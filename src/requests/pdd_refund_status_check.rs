use crate::Request;

use serde::{Deserialize, Serialize};


/// 校验售后单
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddRefundStatusCheck {
    
    /// 20150909-452750051,20150909-452750134 用逗号分开
    #[serde(rename = "order_sns")]
    pub order_sns: Option<String>,
    
}


impl Request for PddRefundStatusCheck {
    fn get_type() -> String {
        "pdd.refund.status.check".to_string()
    }

    fn get_response_name() -> String {
        "refund_status_check_response".to_string()
    }
}
