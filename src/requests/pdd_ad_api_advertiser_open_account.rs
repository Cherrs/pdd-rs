use crate::Request;

use serde::{Deserialize, Serialize};


/// 广告主开户
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiAdvertiserOpenAccount {
    
}


impl Request for PddAdApiAdvertiserOpenAccount {
    fn get_type() -> String {
        "pdd.ad.api.advertiser.open.account".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
