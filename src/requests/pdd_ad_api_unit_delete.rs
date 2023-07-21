use crate::Request;

use serde::{Deserialize, Serialize};


/// 删除单元
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitDelete {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 场景类型。0表示搜索，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: Option<i32>,
    
}


impl Request for PddAdApiUnitDelete {
    fn get_type() -> String {
        "pdd.ad.api.unit.delete".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
