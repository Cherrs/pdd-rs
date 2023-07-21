use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品映射查询接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsGetRelation {
    
    /// 拼多多商品id
    #[serde(rename = "pdd_goods_id")]
    pub pdd_goods_id: Option<Vec<i64>>,
    
}


impl Request for PddGoodsGetRelation {
    fn get_type() -> String {
        "pdd.goods.get.relation".to_string()
    }

    fn get_response_name() -> String {
        "query_goods_relation_response".to_string()
    }
}
