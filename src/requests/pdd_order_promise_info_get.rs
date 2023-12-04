use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询订单承诺信息，用于打单等场景下的承诺信息展示
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOrderPromiseInfoGet {
    
    /// 承诺id
    #[serde(rename = "promise_id")]
    pub promise_id: Option<i64>,
    
}


/// 查询订单承诺信息，用于打单等场景下的承诺信息展示
impl Request for PddOrderPromiseInfoGet {
    fn get_type() -> String {
        "pdd.order.promise.info.get".to_string()
    }

    fn get_response_name() -> String {
        "promise_info_get_response".to_string()
    }
}
