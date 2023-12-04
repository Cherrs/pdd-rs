use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询全店推广API
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCpsMallUnitQuery {
    
}


/// 查询全店推广计划
impl Request for PddGoodsCpsMallUnitQuery {
    fn get_type() -> String {
        "pdd.goods.cps.mall.unit.query".to_string()
    }

    fn get_response_name() -> String {
        "goods_cps_mall_unit_query_response".to_string()
    }
}
