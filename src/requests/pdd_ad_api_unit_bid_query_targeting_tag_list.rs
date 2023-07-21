use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取定向标签数据
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitBidQueryTargetingTagList {
    
}


impl Request for PddAdApiUnitBidQueryTargetingTagList {
    fn get_type() -> String {
        "pdd.ad.api.unit.bid.query.targeting.tag.list".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
