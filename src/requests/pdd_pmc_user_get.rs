use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取用户已开通消息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPmcUserGet {
    
    /// 用户唯一id
    #[serde(rename = "owner_id")]
    pub owner_id: Option<String>,
    
}


/// 获取用户已开通消息
impl Request for PddPmcUserGet {
    fn get_type() -> String {
        "pdd.pmc.user.get".to_string()
    }

    fn get_response_name() -> String {
        "pmc_user_get_response".to_string()
    }
}
