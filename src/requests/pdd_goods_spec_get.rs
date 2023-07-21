use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取商品规格信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSpecGet {
    
    /// 叶子类目ID，必须入参level=3时的cat_id,否则无法返回正确的参数
    #[serde(rename = "cat_id")]
    pub cat_id: Option<i64>,
    
}


impl Request for PddGoodsSpecGet {
    fn get_type() -> String {
        "pdd.goods.spec.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_spec_get_response".to_string()
    }
}
