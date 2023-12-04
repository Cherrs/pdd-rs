use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取拼多多标准地址库
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsAddressGet {
    
}


/// 获取拼多多标准地址库
impl Request for PddLogisticsAddressGet {
    fn get_type() -> String {
        "pdd.logistics.address.get".to_string()
    }

    fn get_response_name() -> String {
        "logistics_address_get_response".to_string()
    }
}
