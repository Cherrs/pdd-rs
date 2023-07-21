use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询广告开户信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiAdvertiserQueryAccountInfo {
    
}


impl Request for PddAdApiAdvertiserQueryAccountInfo {
    fn get_type() -> String {
        "pdd.ad.api.advertiser.query.account.info".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
