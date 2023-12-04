use crate::Request;

use serde::{Deserialize, Serialize};


/// isv查询pdd景区
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTicketScenicGet {
    
    /// 城市编码
    #[serde(rename = "city_code")]
    pub city_code: Option<i64>,
    
    /// 定位类型 1.百度 2.google
    #[serde(rename = "location_type")]
    pub location_type: Option<i32>,
    
    /// 拼多多景区 ID
    #[serde(rename = "scenic_id")]
    pub scenic_id: Option<i64>,
    
    /// 景区简称（至少两个字）
    #[serde(rename = "scenic_name")]
    pub scenic_name: Option<String>,
    
}


/// isv查询pdd景区
impl Request for PddTicketScenicGet {
    fn get_type() -> String {
        "pdd.ticket.scenic.get".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
