use crate::Request;

use serde::{Deserialize, Serialize};


/// 更新优化信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitUpdateOptimizationMessage {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 优化信息
    #[serde(rename = "optimizationMessage")]
    pub optimization_message: Option<OptimizationMessage>,
    
}

/// 更新优化信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OptimizationMessage {
    
    /// 数据积累期出价。当使用OCPX时对该字段赋值。
    #[serde(rename = "accumulationBid")]
    pub accumulation_bid: Option<i64>,
    
    /// 智能投放期出价。当使用OCPX时对该字段赋值。
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: Option<i64>,
    
    /// 优化目标。0表示不优化。1表示优化ROI，2表示优化转化成本。当计划使用智能推广时，该值必须传1；当单元使用自动调价功能(ECPC)时，该值必须传1；当单元使用OCPC功能时，该值必须传2。
    #[serde(rename = "optimizationGoal")]
    pub optimization_goal: Option<i32>,
    
    /// 优化方式。0表示不优化，1表示ECPC，2表示OCPC。当计划使用智能推广时，该值必须传0；当单元使用ECPC时，该值必须传1；当使用OCPC时，该值必须传2。
    #[serde(rename = "optimizationMethod")]
    pub optimization_method: Option<i32>,
    
    /// 可选优化出价列表。当使用OCPX时对该字段赋值。
    #[serde(rename = "optionalOptimizationBidMessageList")]
    pub optional_optimization_bid_message_list: Option<Vec<OptionalOptimizationBidMessageList>>,
    
}

/// 更新优化信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OptionalOptimizationBidMessageList {
    
    /// 可选优化出价价格
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: Option<i64>,
    
    /// 可选优化出价目标。3表示优化店铺关注，4表示优化商品收藏，5表示优化询单
    #[serde(rename = "optimizationGoal")]
    pub optimization_goal: Option<i32>,
    
}


impl Request for PddAdApiUnitUpdateOptimizationMessage {
    fn get_type() -> String {
        "pdd.ad.api.unit.update.optimization.message".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
