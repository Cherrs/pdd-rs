use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取拼多多系统时间
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTimeGet {
    
}


/// 获取拼多多系统时间
impl Request for PddTimeGet {
    fn get_type() -> String {
        "pdd.time.get".to_string()
    }

    fn get_response_name() -> String {
        "time_get_response".to_string()
    }
}
