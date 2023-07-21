use crate::Request;

use serde::{Deserialize, Serialize};


/// 该接口用于客服关闭会话
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsCsSessionClose {
    
    /// pdd会话id
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,
    
    /// 物流公司会话id
    #[serde(rename = "wp_session_id")]
    pub wp_session_id: Option<String>,
    
    /// 样式YYYY-MM-DD HH:MM:SS
    #[serde(rename = "action_time")]
    pub action_time: Option<String>,
    
}


impl Request for PddLogisticsCsSessionClose {
    fn get_type() -> String {
        "pdd.logistics.cs.session.close".to_string()
    }

    fn get_response_name() -> String {
        "logistics_cs_session_close_response".to_string()
    }
}
