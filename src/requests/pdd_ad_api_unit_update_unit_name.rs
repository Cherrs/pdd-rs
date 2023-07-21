use crate::Request;

use serde::{Deserialize, Serialize};


/// 更新单元名称
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitUpdateUnitName {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 单元名称
    #[serde(rename = "adName")]
    pub ad_name: Option<String>,
    
}


impl Request for PddAdApiUnitUpdateUnitName {
    fn get_type() -> String {
        "pdd.ad.api.unit.update.unit.name".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
