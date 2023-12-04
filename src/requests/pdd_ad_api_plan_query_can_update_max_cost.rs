use crate::Request;

use serde::{Deserialize, Serialize};


/// 当天是否允许计划的推广预算
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiPlanQueryCanUpdateMaxCost {
    
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: Option<i64>,
    
}


/// 当天是否允许计划的推广预算
impl Request for PddAdApiPlanQueryCanUpdateMaxCost {
    fn get_type() -> String {
        "pdd.ad.api.plan.query.can.update.max.cost".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
