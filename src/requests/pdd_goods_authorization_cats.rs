use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取当前授权商家可发布的商品类目信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsAuthorizationCats {
    
    /// 默认值=0，值=0时为顶点cat_id,通过树顶级节点获取一级类目
    #[serde(rename = "parent_cat_id")]
    pub parent_cat_id: Option<i64>,
    
}


impl Request for PddGoodsAuthorizationCats {
    fn get_type() -> String {
        "pdd.goods.authorization.cats".to_string()
    }

    fn get_response_name() -> String {
        "goods_auth_cats_get_response".to_string()
    }
}
