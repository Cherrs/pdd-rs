use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询商品长图
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiGoodsQueryLongImages {
    
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddAdApiGoodsQueryLongImages {
    fn get_type() -> String {
        "pdd.ad.api.goods.query.long.images".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
