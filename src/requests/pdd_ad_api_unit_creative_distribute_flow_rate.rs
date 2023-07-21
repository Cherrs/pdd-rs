use crate::Request;

use serde::{Deserialize, Serialize};


/// 智能创意流量比例分配
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitCreativeDistributeFlowRate {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 流量分配比例。万分比
    #[serde(rename = "creativeFlowRate")]
    pub creative_flow_rate: Option<i32>,
    
}


impl Request for PddAdApiUnitCreativeDistributeFlowRate {
    fn get_type() -> String {
        "pdd.ad.api.unit.creative.distribute.flow.rate".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
