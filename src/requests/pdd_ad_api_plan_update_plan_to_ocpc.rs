use crate::Request;

use serde::{Deserialize, Serialize};


/// 支持广告单元切换为ocpx
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiPlanUpdatePlanToOcpc {
    
    /// 单元OCPC信息列表
    #[serde(rename = "adUnitUpdateOcpcMessageList")]
    pub ad_unit_update_ocpc_message_list: Option<Vec<AdUnitUpdateOcpcMessageList>>,
    
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: Option<i64>,
    
    /// 场景类型。0表示搜索。
    #[serde(rename = "scenesType")]
    pub scenes_type: Option<i32>,
    
}

/// 支持广告单元切换为ocpx
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdUnitUpdateOcpcMessageList {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// OCPC信息
    #[serde(rename = "optimizationMessage")]
    pub optimization_message: Option<OptimizationMessage>,
    
}

/// 支持广告单元切换为ocpx
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OptimizationMessage {
    
    /// 智能投放期出价
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: Option<i64>,
    
    /// 优化目标。单元使用OCPC功能时，该值必须传2。
    #[serde(rename = "optimizationGoal")]
    pub optimization_goal: Option<i32>,
    
    /// 优化方式。当使用OCPC时，该值必须传2。
    #[serde(rename = "optimizationMethod")]
    pub optimization_method: Option<i32>,
    
}


impl Request for PddAdApiPlanUpdatePlanToOcpc {
    fn get_type() -> String {
        "pdd.ad.api.plan.update.plan.to.ocpc".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
