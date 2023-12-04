use crate::Request;

use serde::{Deserialize, Serialize};


/// 仓库详细信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddExpressDepotInfoGet {
    
    /// 仓库id
    #[serde(rename = "depot_id")]
    pub depot_id: Option<i64>,
    
}


/// 仓库详细信息
impl Request for PddExpressDepotInfoGet {
    fn get_type() -> String {
        "pdd.express.depot.info.get".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
