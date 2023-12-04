use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询商品推广API
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCpsUnitQuery {
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


/// 查询商品推广API
impl Request for PddGoodsCpsUnitQuery {
    fn get_type() -> String {
        "pdd.goods.cps.unit.query".to_string()
    }

    fn get_response_name() -> String {
        "coupon_vo".to_string()
    }
}
