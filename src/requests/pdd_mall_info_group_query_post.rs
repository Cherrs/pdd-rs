use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询店铺下门店组列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoGroupQueryPost {
    
    /// 第几页
    #[serde(rename = "page_number")]
    pub page_number: Option<i32>,
    
    /// 每页多少个
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
}


impl Request for PddMallInfoGroupQueryPost {
    fn get_type() -> String {
        "pdd.mall.info.group.query.post".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
