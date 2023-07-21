use crate::Request;

use serde::{Deserialize, Serialize};


/// 消息队列积压数量查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPmcAccrueQuery {
    
}


impl Request for PddPmcAccrueQuery {
    fn get_type() -> String {
        "pdd.pmc.accrue.query".to_string()
    }

    fn get_response_name() -> String {
        "pmc_user_get_response".to_string()
    }
}
