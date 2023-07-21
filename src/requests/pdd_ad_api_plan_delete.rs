use crate::Request;

use serde::{Deserialize, Serialize};


/// 删除计划
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiPlanDelete {
    
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: Option<i64>,
    
    /// 场景类型，0-搜索，2-展示
    #[serde(rename = "scenesType")]
    pub scenes_type: Option<i32>,
    
}


impl Request for PddAdApiPlanDelete {
    fn get_type() -> String {
        "pdd.ad.api.plan.delete".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
