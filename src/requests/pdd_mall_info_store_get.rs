use crate::Request;

use serde::{Deserialize, Serialize};


/// 开放平台根据条件查询门店信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoStoreGet {
    
    /// 市
    #[serde(rename = "city")]
    pub city: Option<String>,
    
    /// 区
    #[serde(rename = "district")]
    pub district: Option<String>,
    
    /// 分页
    #[serde(rename = "page_number")]
    pub page_number: Option<i32>,
    
    /// 分页大小
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 省
    #[serde(rename = "province")]
    pub province: Option<String>,
    
    /// 门店Id
    #[serde(rename = "store_id")]
    pub store_id: Option<i64>,
    
    /// 门店名称
    #[serde(rename = "store_name")]
    pub store_name: Option<String>,
    
    /// 门店自有编号
    #[serde(rename = "store_number")]
    pub store_number: Option<String>,
    
}


/// 开放平台根据条件查询门店信息
impl Request for PddMallInfoStoreGet {
    fn get_type() -> String {
        "pdd.mall.info.store.get".to_string()
    }

    fn get_response_name() -> String {
        "res".to_string()
    }
}
