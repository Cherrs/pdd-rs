use crate::Request;

use serde::{Deserialize, Serialize};


/// 门店组添加门店
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoGroupAddStorePost {
    
    /// 店铺ID
    #[serde(rename = "group_id")]
    pub group_id: Option<i64>,
    
    /// 门店ID列表
    #[serde(rename = "store_id_list")]
    pub store_id_list: Option<Vec<i64>>,
    
}


impl Request for PddMallInfoGroupAddStorePost {
    fn get_type() -> String {
        "pdd.mall.info.group.add.store.post".to_string()
    }

    fn get_response_name() -> String {
        "mall_info_group_add_store_post_response".to_string()
    }
}
