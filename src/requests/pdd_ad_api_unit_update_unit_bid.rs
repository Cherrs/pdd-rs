use crate::Request;

use serde::{Deserialize, Serialize};


/// 更新单元出价
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitUpdateUnitBid {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 出价不得为空。单位厘。
    #[serde(rename = "bid")]
    pub bid: Option<i64>,
    
}


/// 更新单元出价
impl Request for PddAdApiUnitUpdateUnitBid {
    fn get_type() -> String {
        "pdd.ad.api.unit.update.unit.bid".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
