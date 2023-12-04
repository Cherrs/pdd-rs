use crate::Request;

use serde::{Deserialize, Serialize};


/// 删除定向/资源位
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitBidDelete {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 出价Id列表
    #[serde(rename = "bidIds")]
    pub bid_ids: Option<Vec<i64>>,
    
}


/// 删除定向/资源位
impl Request for PddAdApiUnitBidDelete {
    fn get_type() -> String {
        "pdd.ad.api.unit.bid.delete".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
