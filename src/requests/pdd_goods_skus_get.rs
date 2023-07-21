use crate::Request;

use serde::{Deserialize, Serialize};


/// 库存同步、改价等场景
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSkusGet {
    
    /// 商品Id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// sku id
    #[serde(rename = "sku_id")]
    pub sku_id: Option<i64>,
    
}


impl Request for PddGoodsSkusGet {
    fn get_type() -> String {
        "pdd.goods.skus.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_sku_get_response".to_string()
    }
}
