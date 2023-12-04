use crate::Request;

use serde::{Deserialize, Serialize};


/// 家电分仓库存-列表查询货品
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockWareList {
    
    /// 货品id
    #[serde(rename = "id")]
    pub id: Option<i64>,
    
    /// 货品编码
    #[serde(rename = "ware_sn")]
    pub ware_sn: Option<String>,
    
    /// 货品名称
    #[serde(rename = "ware_name")]
    pub ware_name: Option<String>,
    
    /// 类型 0:单独货品。1:组合货品
    #[serde(rename = "ware_type")]
    pub ware_type: Option<i32>,
    
    /// 页数，从1开始
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 每页记录数
    #[serde(rename = "size")]
    pub size: Option<i32>,
    
}


/// 家电分仓库存-列表查询货品
impl Request for PddStockWareList {
    fn get_type() -> String {
        "pdd.stock.ware.list".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
