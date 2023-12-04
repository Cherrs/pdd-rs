use crate::Request;

use serde::{Deserialize, Serialize};


/// 拼多多向物流公司订阅指定运单号的物流轨迹详情
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsCoTrackSub {
    
    /// 快递公司伙伴ID
    #[serde(rename = "ship_id")]
    pub ship_id: Option<String>,
    
    /// 消息体
    #[serde(rename = "data")]
    pub data: Option<String>,
    
}


/// 拼多多向物流公司订阅指定运单号的物流轨迹详情
impl Request for PddLogisticsCoTrackSub {
    fn get_type() -> String {
        "pdd.logistics.co.track.sub".to_string()
    }

    fn get_response_name() -> String {
        "ship_id".to_string()
    }
}
