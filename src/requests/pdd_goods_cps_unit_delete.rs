use crate::Request;

use serde::{Deserialize, Serialize};


/// 删除单品计划功能
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCpsUnitDelete {
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddGoodsCpsUnitDelete {
    fn get_type() -> String {
        "pdd.goods.cps.unit.delete".to_string()
    }

    fn get_response_name() -> String {
        "goods_cps_unit_delete_response".to_string()
    }
}
