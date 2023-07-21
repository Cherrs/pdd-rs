use crate::Request;

use serde::{Deserialize, Serialize};


/// 删除门店组
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoGroupDeletePost {
    
    /// 门店组id列表
    #[serde(rename = "group_id_list")]
    pub group_id_list: Option<Vec<i32>>,
    
}


impl Request for PddMallInfoGroupDeletePost {
    fn get_type() -> String {
        "pdd.mall.info.group.delete.post".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
