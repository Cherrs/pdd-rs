use crate::Request;

use serde::{Deserialize, Serialize};


/// 修改全店推广计划
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCpsMallUnitChange {
    
    /// 全店推广计划佣金比（千分比）
    #[serde(rename = "rate")]
    pub rate: Option<i32>,
    
}


impl Request for PddGoodsCpsMallUnitChange {
    fn get_type() -> String {
        "pdd.goods.cps.mall.unit.change".to_string()
    }

    fn get_response_name() -> String {
        "goods_cps_mall_unit_change_response".to_string()
    }
}
