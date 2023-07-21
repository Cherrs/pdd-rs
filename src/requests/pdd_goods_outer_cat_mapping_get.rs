use crate::Request;

use serde::{Deserialize, Serialize};


/// 类目预测
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsOuterCatMappingGet {
    
    /// 外部叶子类目id
    #[serde(rename = "outer_cat_id")]
    pub outer_cat_id: Option<i64>,
    
    /// 外部叶子类目名称
    #[serde(rename = "outer_cat_name")]
    pub outer_cat_name: Option<String>,
    
    /// 外部商品名称
    #[serde(rename = "outer_goods_name")]
    pub outer_goods_name: Option<String>,
    
}


impl Request for PddGoodsOuterCatMappingGet {
    fn get_type() -> String {
        "pdd.goods.outer.cat.mapping.get".to_string()
    }

    fn get_response_name() -> String {
        "outer_cat_mapping_get_response".to_string()
    }
}
