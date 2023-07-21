use crate::Request;

use serde::{Deserialize, Serialize};


/// 该接口用于获取客户与机器人的聊天记录。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsCsHistoryMessageGet {
    
    /// pdd会话id
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,
    
}


impl Request for PddLogisticsCsHistoryMessageGet {
    fn get_type() -> String {
        "pdd.logistics.cs.history.message.get".to_string()
    }

    fn get_response_name() -> String {
        "logistics_cs_history_message_get_response".to_string()
    }
}
