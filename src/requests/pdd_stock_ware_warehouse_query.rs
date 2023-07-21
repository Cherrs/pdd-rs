use crate::Request;

use serde::{Deserialize, Serialize};


/// 通过货品编码查询货品和库存信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockWareWarehouseQuery {
    
    /// 请求对象
    #[serde(rename = "request")]
    pub request: Option<PddStockWareWarehouseQueryRequest>,
    
}

/// 通过货品编码查询货品和库存信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockWareWarehouseQueryRequest {
    
    /// 当前页数
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 页显示数据条数
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 货品编码列表
    #[serde(rename = "ware_sn_list")]
    pub ware_sn_list: Option<Vec<String>>,
    
}


impl Request for PddStockWareWarehouseQuery {
    fn get_type() -> String {
        "pdd.stock.ware.warehouse.query".to_string()
    }

    fn get_response_name() -> String {
        "result".to_string()
    }
}
