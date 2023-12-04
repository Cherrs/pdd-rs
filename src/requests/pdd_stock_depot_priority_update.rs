use crate::Request;

use serde::{Deserialize, Serialize};


/// 批量更新仓库优先级
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockDepotPriorityUpdate {
    
    /// 示例：[{"depot_id":1,"province_id":12,"city_id":34,"district_id":56,"priority":5}]
    #[serde(rename = "priority_list")]
    pub priority_list: Option<Vec<PriorityList>>,
    
}

/// 批量更新仓库优先级
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PriorityList {
    
    /// 优先级 范围是1 - 999
    #[serde(rename = "priority")]
    pub priority: Option<i32>,
    
    /// 区id
    #[serde(rename = "district_id")]
    pub district_id: Option<i32>,
    
    /// 市id
    #[serde(rename = "city_id")]
    pub city_id: Option<i32>,
    
    /// 省id
    #[serde(rename = "province_id")]
    pub province_id: Option<i32>,
    
    /// 仓库id
    #[serde(rename = "depot_id")]
    pub depot_id: Option<i64>,
    
}


/// 批量更新仓库优先级
impl Request for PddStockDepotPriorityUpdate {
    fn get_type() -> String {
        "pdd.stock.depot.priority.update".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
