use crate::Request;

use serde::{Deserialize, Serialize};


/// 收到分配，更新地址消息，使用该接口拉取订单详情
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFdsOrderGet {
    
    /// 入参信息
    #[serde(rename = "param_fds_order_get_request")]
    pub param_fds_order_get_request: Option<ParamFdsOrderGetRequest>,
    
}

/// 收到分配，更新地址消息，使用该接口拉取订单详情
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParamFdsOrderGetRequest {
    
    /// 代打店铺id
    #[serde(rename = "mall_mask_id")]
    pub mall_mask_id: Option<String>,
    
    /// 代打订单号
    #[serde(rename = "order_mask_sn")]
    pub order_mask_sn: Option<String>,
    
}


impl Request for PddFdsOrderGet {
    fn get_type() -> String {
        "pdd.fds.order.get".to_string()
    }

    fn get_response_name() -> String {
        "pdd_fds_order_get_response".to_string()
    }
}
