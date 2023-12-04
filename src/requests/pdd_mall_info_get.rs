use crate::Request;

use serde::{Deserialize, Serialize};


/// 店铺信息接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoGet {
    
}


/// 通过此接口获取店铺信息
impl Request for PddMallInfoGet {
    fn get_type() -> String {
        "pdd.mall.info.get".to_string()
    }

    fn get_response_name() -> String {
        "mall_info_get_response".to_string()
    }
}
