use crate::Request;

use serde::{Deserialize, Serialize};


/// 该接口用于物流客服系统创建以及同步会话状态。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsCsSessionStart {
    
    /// pdd会话id
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,
    
    /// 物流公司会话id
    #[serde(rename = "wp_session_id")]
    pub wp_session_id: Option<String>,
    
    /// 样式YYYY-MM-DD HH:MM:SS
    #[serde(rename = "action_time")]
    pub action_time: Option<String>,
    
    /// 可选值：1：已分配 2：排队中 3：分配异常
    #[serde(rename = "biz_type")]
    pub biz_type: Option<i32>,
    
    /// 客服id，biz_type为1时必填
    #[serde(rename = "dealer_id")]
    pub dealer_id: Option<String>,
    
    /// 队列id，biz_type为1时必填
    #[serde(rename = "queue_id")]
    pub queue_id: Option<String>,
    
    /// 网点名，biz_type为1时必填
    #[serde(rename = "queue_name")]
    pub queue_name: Option<String>,
    
    /// 排队位置，biz_type为2时必填
    #[serde(rename = "queue_index")]
    pub queue_index: Option<i32>,
    
    /// 分配遇到的异常，示例：33222，biz_type为3时不为空
    #[serde(rename = "exception_code")]
    pub exception_code: Option<i32>,
    
    /// 物流客服系统遇到的异常，biz_type为3时不为空
    #[serde(rename = "exception_msg")]
    pub exception_msg: Option<String>,
    
    /// 接待的网点地址，biz_type为1时必填，示例: ”河南省”
    #[serde(rename = "queue_address")]
    pub queue_address: Option<String>,
    
}


impl Request for PddLogisticsCsSessionStart {
    fn get_type() -> String {
        "pdd.logistics.cs.session.start".to_string()
    }

    fn get_response_name() -> String {
        "logistics_cs_session_start_response".to_string()
    }
}
