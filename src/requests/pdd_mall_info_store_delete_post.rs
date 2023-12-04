use crate::Request;

use serde::{Deserialize, Serialize};


/// 删除店铺门店
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoStoreDeletePost {
    
    /// 门店ID列表
    #[serde(rename = "store_id_list")]
    pub store_id_list: Option<Vec<i64>>,
    
}


/// 删除店铺门店
impl Request for PddMallInfoStoreDeletePost {
    fn get_type() -> String {
        "pdd.mall.info.store.delete.post".to_string()
    }

    fn get_response_name() -> String {
        "mall_info_store_delete_post_response".to_string()
    }
}
