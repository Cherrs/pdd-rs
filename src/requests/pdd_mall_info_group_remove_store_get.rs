use crate::Request;

use serde::{Deserialize, Serialize};


/// 门店组删除门店
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoGroupRemoveStoreGet {
    
    /// 店铺ID
    #[serde(rename = "group_id")]
    pub group_id: Option<i64>,
    
    /// 门店ID列表
    #[serde(rename = "store_id_list")]
    pub store_id_list: Option<Vec<i64>>,
    
}


/// 门店组删除门店
impl Request for PddMallInfoGroupRemoveStoreGet {
    fn get_type() -> String {
        "pdd.mall.info.group.remove.store.get".to_string()
    }

    fn get_response_name() -> String {
        "mall_info_group_remove_store_get_response".to_string()
    }
}
