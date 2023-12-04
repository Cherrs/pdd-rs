use crate::Request;

use serde::{Deserialize, Serialize};


/// 新增门店组
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoGroupAddPost {
    
    /// 门店组名称
    #[serde(rename = "group_name")]
    pub group_name: Option<String>,
    
}


/// 新增门店组
impl Request for PddMallInfoGroupAddPost {
    fn get_type() -> String {
        "pdd.mall.info.group.add.post".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
