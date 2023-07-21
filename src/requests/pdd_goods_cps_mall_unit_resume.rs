use crate::Request;

use serde::{Deserialize, Serialize};


/// 恢复全店推广API
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCpsMallUnitResume {
    
}


impl Request for PddGoodsCpsMallUnitResume {
    fn get_type() -> String {
        "pdd.goods.cps.mall.unit.resume".to_string()
    }

    fn get_response_name() -> String {
        "goods_cps_mall_unit_resume_response".to_string()
    }
}
