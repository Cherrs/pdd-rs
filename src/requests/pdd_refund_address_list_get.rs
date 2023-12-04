use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取商家退货地址库
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddRefundAddressListGet {
    
}


/// 获取商家退货地址库
impl Request for PddRefundAddressListGet {
    fn get_type() -> String {
        "pdd.refund.address.list.get".to_string()
    }

    fn get_response_name() -> String {
        "refund_address_list_get_response".to_string()
    }
}
