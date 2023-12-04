use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询广告主详情信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiAdvertiserQueryDetail {
    
}


/// 查询广告主详情信息
impl Request for PddAdApiAdvertiserQueryDetail {
    fn get_type() -> String {
        "pdd.ad.api.advertiser.query.detail".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
