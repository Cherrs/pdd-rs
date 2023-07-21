use crate::Request;

use serde::{Deserialize, Serialize};


/// 暂停全店推广API
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCpsMallUnitPause {
    
}


impl Request for PddGoodsCpsMallUnitPause {
    fn get_type() -> String {
        "pdd.goods.cps.mall.unit.pause".to_string()
    }

    fn get_response_name() -> String {
        "goods_cps_mall_unit_pause_response".to_string()
    }
}
