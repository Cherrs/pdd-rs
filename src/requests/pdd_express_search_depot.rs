use crate::Request;

use serde::{Deserialize, Serialize};


/// 根据仓库名称和仓库编码查询仓库
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddExpressSearchDepot {
    
    /// 仓库编码
    #[serde(rename = "code")]
    pub code: Option<String>,
    
    /// 10 分页数据size
    #[serde(rename = "length")]
    pub length: Option<i32>,
    
    /// 仓库名称
    #[serde(rename = "name")]
    pub name: Option<String>,
    
    /// 0 分页数据起始位置
    #[serde(rename = "start")]
    pub start: Option<i32>,
    
}


impl Request for PddExpressSearchDepot {
    fn get_type() -> String {
        "pdd.express.search.depot".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
