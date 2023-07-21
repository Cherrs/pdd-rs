use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取可用资源位
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitBidQueryBaseLocationProfile {
    
}


impl Request for PddAdApiUnitBidQueryBaseLocationProfile {
    fn get_type() -> String {
        "pdd.ad.api.unit.bid.query.base.location.profile".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
