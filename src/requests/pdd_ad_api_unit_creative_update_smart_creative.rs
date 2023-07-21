use crate::Request;

use serde::{Deserialize, Serialize};


/// 更新智能创意
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitCreativeUpdateSmartCreative {
    
    /// 标题
    #[serde(rename = "text")]
    pub text: Option<String>,
    
    /// 创意单元Id
    #[serde(rename = "unitCreativeId")]
    pub unit_creative_id: Option<i64>,
    
}


impl Request for PddAdApiUnitCreativeUpdateSmartCreative {
    fn get_type() -> String {
        "pdd.ad.api.unit.creative.update.smart.creative".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
