use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询智能创意流量分配比例,万分比
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitCreativeQueryFlowRate {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
}


impl Request for PddAdApiUnitCreativeQueryFlowRate {
    fn get_type() -> String {
        "pdd.ad.api.unit.creative.query.flow.rate".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
