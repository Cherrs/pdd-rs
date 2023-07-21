use crate::Request;

use serde::{Deserialize, Serialize};


/// 该接口用于查询虚拟业务订单的特有字段
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOrderVirtualInformationGet {
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
}


impl Request for PddOrderVirtualInformationGet {
    fn get_type() -> String {
        "pdd.order.virtual.information.get".to_string()
    }

    fn get_response_name() -> String {
        "order_virtual_information_response".to_string()
    }
}
