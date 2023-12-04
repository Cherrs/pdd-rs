use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询店铺是否签署多多进宝协议接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallCpsProtocolStatusQuery {
    
}


/// 查询店铺是否签署多多进宝协议接口
impl Request for PddMallCpsProtocolStatusQuery {
    fn get_type() -> String {
        "pdd.mall.cps.protocol.status.query".to_string()
    }

    fn get_response_name() -> String {
        "mall_cps_protocol_status_query_response".to_string()
    }
}
