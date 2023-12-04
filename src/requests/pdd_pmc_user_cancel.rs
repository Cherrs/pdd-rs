use crate::Request;

use serde::{Deserialize, Serialize};


/// 取消用户的消息服务
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPmcUserCancel {
    
    /// 用户唯一id
    #[serde(rename = "owner_id")]
    pub owner_id: Option<String>,
    
}


/// 取消用户的消息服务
impl Request for PddPmcUserCancel {
    fn get_type() -> String {
        "pdd.pmc.user.cancel".to_string()
    }

    fn get_response_name() -> String {
        "tmc_user_cancel_response".to_string()
    }
}
