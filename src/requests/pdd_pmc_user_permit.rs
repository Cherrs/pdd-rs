use crate::Request;

use serde::{Deserialize, Serialize};


/// 为已授权的用户开通消息服务
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPmcUserPermit {
    
    /// 消息主题列表，用半角逗号分隔。当用户订阅的topic是应用订阅的子集时才需要设置，不设置表示继承应用所订阅的所有topic，一般情况建议不要设置。
    #[serde(rename = "topics")]
    pub topics: Option<String>,
    
}


/// 为已授权的用户开通消息服务
impl Request for PddPmcUserPermit {
    fn get_type() -> String {
        "pdd.pmc.user.permit".to_string()
    }

    fn get_response_name() -> String {
        "pmc_user_permit_response".to_string()
    }
}
