use crate::Request;

use serde::{Deserialize, Serialize};


/// 判断是否对商家展示某个通知
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallNotificationTypeShowCheck {
    
    /// 通知类型
    #[serde(rename = "notification_type")]
    pub notification_type: Option<String>,
    
}


impl Request for PddMallNotificationTypeShowCheck {
    fn get_type() -> String {
        "pdd.mall.notification.type.show.check".to_string()
    }

    fn get_response_name() -> String {
        "result".to_string()
    }
}
