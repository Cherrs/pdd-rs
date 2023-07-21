use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询商品素材列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsMaterialQuery {
    
    /// 商品id列表
    #[serde(rename = "goods_id_list")]
    pub goods_id_list: Option<Vec<i64>>,
    
    /// 素材类型列表
    #[serde(rename = "type_list")]
    pub type_list: Option<Vec<i64>>,
    
}


impl Request for PddGoodsMaterialQuery {
    fn get_type() -> String {
        "pdd.goods.material.query".to_string()
    }

    fn get_response_name() -> String {
        "material_list".to_string()
    }
}
