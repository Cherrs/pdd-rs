use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询门店组下门店
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoGroupListStoreGet {
    
    /// 门店组ID
    #[serde(rename = "group_id")]
    pub group_id: Option<i64>,
    
    /// 分页页码
    #[serde(rename = "page_number")]
    pub page_number: Option<i32>,
    
    /// 分页大小
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
}


/// 查询门店组下门店
impl Request for PddMallInfoGroupListStoreGet {
    fn get_type() -> String {
        "pdd.mall.info.group.list.store.get".to_string()
    }

    fn get_response_name() -> String {
        "mall_info_group_list_store_get_response".to_string()
    }
}
