use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询订单详情
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkOauthOrderDetailGet {
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 订单类型：1-推广订单；2-直播间订单
    #[serde(rename = "query_order_type")]
    pub query_order_type: Option<i32>,
    
}


/// 查询订单详情
impl Request for PddDdkOauthOrderDetailGet {
    fn get_type() -> String {
        "pdd.ddk.oauth.order.detail.get".to_string()
    }

    fn get_response_name() -> String {
        "order_detail_response".to_string()
    }
}
