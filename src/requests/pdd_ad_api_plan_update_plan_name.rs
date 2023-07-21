use crate::Request;

use serde::{Deserialize, Serialize};


/// 更新计划名称
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiPlanUpdatePlanName {
    
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: Option<i64>,
    
    /// 计划名称
    #[serde(rename = "planName")]
    pub plan_name: Option<String>,
    
}


impl Request for PddAdApiPlanUpdatePlanName {
    fn get_type() -> String {
        "pdd.ad.api.plan.update.plan.name".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
