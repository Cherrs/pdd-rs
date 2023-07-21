use crate::Request;

use serde::{Deserialize, Serialize};


/// 仓库列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddExpressDepotListGet {
    
    /// 分页数据size
    #[serde(rename = "length")]
    pub length: Option<i64>,
    
    /// 分页数据起始位置
    #[serde(rename = "start")]
    pub start: Option<i64>,
    
}


impl Request for PddExpressDepotListGet {
    fn get_type() -> String {
        "pdd.express.depot.list.get".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
