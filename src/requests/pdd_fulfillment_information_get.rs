use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询单个跨境全托管发货单详情（只能获取到成交时间三个月以内的交易信息）
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFulfillmentInformationGet {
    
    /// 跨境全托管发货单号
    #[serde(rename = "fulfillment_sn")]
    pub fulfillment_sn: Option<String>,
    
}


impl Request for PddFulfillmentInformationGet {
    fn get_type() -> String {
        "pdd.fulfillment.information.get".to_string()
    }

    fn get_response_name() -> String {
        "fulfillment_info_get_response".to_string()
    }
}
