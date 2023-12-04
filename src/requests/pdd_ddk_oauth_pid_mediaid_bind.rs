use crate::Request;

use serde::{Deserialize, Serialize};


/// 批量对pid与媒体id进行绑定
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkOauthPidMediaidBind {
    
    /// 媒体id
    #[serde(rename = "media_id")]
    pub media_id: Option<i64>,
    
    /// 推广位列表，例如：["60005_612"]，最多支持同时传入1000个
    #[serde(rename = "pid_list")]
    pub pid_list: Option<Vec<String>>,
    
}


/// 批量对pid与媒体id进行绑定
impl Request for PddDdkOauthPidMediaidBind {
    fn get_type() -> String {
        "pdd.ddk.oauth.pid.mediaid.bind".to_string()
    }

    fn get_response_name() -> String {
        "p_id_bind_response".to_string()
    }
}
