use crate::Request;

use serde::{Deserialize, Serialize};


/// 该接口用于客服给客户发消息，发消息的前提是有客服分配。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsCsMessageSend {
    
    /// pdd会话id
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,
    
    /// 物流公司会话id
    #[serde(rename = "wp_session_id")]
    pub wp_session_id: Option<String>,
    
    /// 样式YYYY-MM-DD HH:MM:SS
    #[serde(rename = "action_time")]
    pub action_time: Option<String>,
    
    /// 0：文本1：图片
    #[serde(rename = "message_type")]
    pub message_type: Option<i32>,
    
    /// message_type为0时不为空
    #[serde(rename = "text")]
    pub text: Option<String>,
    
    /// message_type为1时不为空
    #[serde(rename = "attach")]
    pub attach: Option<String>,
    
    /// message_type为1时不为空
    #[serde(rename = "preview")]
    pub preview: Option<String>,
    
}


impl Request for PddLogisticsCsMessageSend {
    fn get_type() -> String {
        "pdd.logistics.cs.message.send".to_string()
    }

    fn get_response_name() -> String {
        "logistics_cs_message_send_response".to_string()
    }
}
