use crate::Request;

use serde::{Deserialize, Serialize};


/// 设置全店推广API
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCpsMallUnitCreate {
    
    /// 合作方code
    #[serde(rename = "erp_code")]
    pub erp_code: Option<String>,
    
    /// 佣金比（千分比）
    #[serde(rename = "rate")]
    pub rate: Option<i64>,
    
}


impl Request for PddGoodsCpsMallUnitCreate {
    fn get_type() -> String {
        "pdd.goods.cps.mall.unit.create".to_string()
    }

    fn get_response_name() -> String {
        "is_create_success".to_string()
    }
}
