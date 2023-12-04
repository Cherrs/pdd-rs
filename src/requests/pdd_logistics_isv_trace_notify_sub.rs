use crate::Request;

use serde::{Deserialize, Serialize};


/// 商家在ISV发货成功之后，ISV通过调用订阅接口订阅轨迹推送消息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsIsvTraceNotifySub {
    
    /// 快递公司编码
    #[serde(rename = "ship_code")]
    pub ship_code: Option<String>,
    
    /// 收件人手机
    #[serde(rename = "tel")]
    pub tel: Option<String>,
    
    /// 快递单号
    #[serde(rename = "track_no")]
    pub track_no: Option<String>,
    
}


/// 商家在ISV发货成功之后，ISV通过调用订阅接口订阅轨迹推送消息
impl Request for PddLogisticsIsvTraceNotifySub {
    fn get_type() -> String {
        "pdd.logistics.isv.trace.notify.sub".to_string()
    }

    fn get_response_name() -> String {
        "logistics_isv_trace_notify_sub".to_string()
    }
}
