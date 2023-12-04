use crate::Request;

use serde::{Deserialize, Serialize};


/// 消息队列积压数量查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPmcAccrueQuery {
    
}


/// 服务商订阅消息后，查询当前积压未消费消息数量（7日内）。
impl Request for PddPmcAccrueQuery {
    fn get_type() -> String {
        "pdd.pmc.accrue.query".to_string()
    }

    fn get_response_name() -> String {
        "pmc_user_get_response".to_string()
    }
}
