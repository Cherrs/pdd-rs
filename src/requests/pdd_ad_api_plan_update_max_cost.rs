use crate::Request;

use serde::{Deserialize, Serialize};


/// 更新日消耗上限
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiPlanUpdateMaxCost {
    
    /// 日消耗上限。单位厘。
    #[serde(rename = "maxCost")]
    pub max_cost: Option<i64>,
    
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: Option<i64>,
    
}


impl Request for PddAdApiPlanUpdateMaxCost {
    fn get_type() -> String {
        "pdd.ad.api.plan.update.max.cost".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
