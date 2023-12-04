use crate::Request;

use serde::{Deserialize, Serialize};


/// 编辑门店组
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoGroupUpdatePost {
    
    /// 门店组id
    #[serde(rename = "group_id")]
    pub group_id: Option<i32>,
    
    /// 门店组名称
    #[serde(rename = "group_name")]
    pub group_name: Option<String>,
    
}


/// 编辑门店组
impl Request for PddMallInfoGroupUpdatePost {
    fn get_type() -> String {
        "pdd.mall.info.group.update.post".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
