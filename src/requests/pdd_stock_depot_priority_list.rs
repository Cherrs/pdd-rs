use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取仓库优先级列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockDepotPriorityList {
    
    /// 省id
    #[serde(rename = "province_id")]
    pub province_id: Option<i32>,
    
    /// 市id
    #[serde(rename = "city_id")]
    pub city_id: Option<i32>,
    
    /// 区id
    #[serde(rename = "district_id")]
    pub district_id: Option<i32>,
    
    /// 仓库编码
    #[serde(rename = "depot_code")]
    pub depot_code: Option<String>,
    
    /// 每页数据显示数量
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 当前页数 从1开始
    #[serde(rename = "page_num")]
    pub page_num: Option<i32>,
    
}


impl Request for PddStockDepotPriorityList {
    fn get_type() -> String {
        "pdd.stock.depot.priority.list".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
