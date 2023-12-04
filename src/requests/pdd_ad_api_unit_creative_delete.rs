use crate::Request;

use serde::{Deserialize, Serialize};


/// 删除创意
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitCreativeDelete {
    
    /// 创意单元Id
    #[serde(rename = "unitCreativeId")]
    pub unit_creative_id: Option<i64>,
    
}


/// 删除创意
impl Request for PddAdApiUnitCreativeDelete {
    fn get_type() -> String {
        "pdd.ad.api.unit.creative.delete".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
