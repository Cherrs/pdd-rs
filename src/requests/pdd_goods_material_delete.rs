use crate::Request;

use serde::{Deserialize, Serialize};


/// 删除素材
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsMaterialDelete {
    
    /// 素材id
    #[serde(rename = "material_id")]
    pub material_id: Option<i64>,
    
}


impl Request for PddGoodsMaterialDelete {
    fn get_type() -> String {
        "pdd.goods.material.delete".to_string()
    }

    fn get_response_name() -> String {
        "error_code".to_string()
    }
}
