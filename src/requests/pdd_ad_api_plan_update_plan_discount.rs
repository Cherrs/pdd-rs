use crate::Request;

use serde::{Deserialize, Serialize};


/// 更新分时折扣
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Discounts {
    
    /// 小时。0-23分别表示第1个小时到第24个小时。
    #[serde(rename = "index")]
    pub index: Option<i32>,
    
    /// 折扣比例。千分比（即rate等于1000表示比例100%）。
    #[serde(rename = "rate")]
    pub rate: Option<i32>,
    
}

/// 更新分时折扣
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiPlanUpdatePlanDiscount {
    
    /// 分时折扣
    #[serde(rename = "planDiscount")]
    pub plan_discount: Option<PlanDiscount>,
    
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: Option<i64>,
    
}

/// 更新分时折扣
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PlanDiscount {
    
    /// 分时折扣配置列表
    #[serde(rename = "discounts")]
    pub discounts: Option<Vec<Discounts>>,
    
}


/// 更新分时折扣
impl Request for PddAdApiPlanUpdatePlanDiscount {
    fn get_type() -> String {
        "pdd.ad.api.plan.update.plan.discount".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
