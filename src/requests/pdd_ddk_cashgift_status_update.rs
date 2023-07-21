use crate::Request;

use serde::{Deserialize, Serialize};


/// 多多客暂停或恢复多多礼金推广
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkCashgiftStatusUpdate {
    
    /// 多多礼金ID
    #[serde(rename = "cash_gift_id")]
    pub cash_gift_id: Option<i64>,
    
    /// 礼金更新类型：0-停止礼金推广，1-恢复礼金推广
    #[serde(rename = "update_type")]
    pub update_type: Option<i32>,
    
}


impl Request for PddDdkCashgiftStatusUpdate {
    fn get_type() -> String {
        "pdd.ddk.cashgift.status.update".to_string()
    }

    fn get_response_name() -> String {
        "update_cashgift_response".to_string()
    }
}
