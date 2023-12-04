use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询商品全站推广建议出价
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitTrListGoodsBidSuggest {
    
    /// 商品id列表
    #[serde(rename = "goodsIds")]
    pub goods_ids: Option<Vec<i64>>,
    
}


/// 查询商品全站推广建议出价
impl Request for PddAdApiUnitTrListGoodsBidSuggest {
    fn get_type() -> String {
        "pdd.ad.api.unit.tr.list.goods.bid.suggest".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
