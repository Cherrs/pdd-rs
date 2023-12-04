use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取拼多多标准商品类目信息（请使用pdd.goods.authorization.cats接口获取商家可发布类目）
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCatsGet {
    
    /// 值=0时为顶点cat_id,通过树顶级节点获取cat树
    #[serde(rename = "parent_cat_id")]
    pub parent_cat_id: Option<i64>,
    
}


/// 获取拼多多标准商品类目信息（请使用pdd.goods.authorization.cats接口获取商家可发布类目）
impl Request for PddGoodsCatsGet {
    fn get_type() -> String {
        "pdd.goods.cats.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_cats_get_response".to_string()
    }
}
