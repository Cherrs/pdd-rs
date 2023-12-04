use crate::Request;

use serde::{Deserialize, Serialize};


/// 校验是否能创建计划
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiPlanQueryCanCreateAdPlan {
    
    /// 单元名称
    #[serde(rename = "planName")]
    pub plan_name: Option<String>,
    
    /// 场景类型。0表示搜索，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: Option<i32>,
    
}


/// 校验是否能创建计划
impl Request for PddAdApiPlanQueryCanCreateAdPlan {
    fn get_type() -> String {
        "pdd.ad.api.plan.query.can.create.ad.plan".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
