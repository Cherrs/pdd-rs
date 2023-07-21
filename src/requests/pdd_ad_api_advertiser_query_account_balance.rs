use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询广告主账户余额
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiAdvertiserQueryAccountBalance {
    
}


impl Request for PddAdApiAdvertiserQueryAccountBalance {
    fn get_type() -> String {
        "pdd.ad.api.advertiser.query.account.balance".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
