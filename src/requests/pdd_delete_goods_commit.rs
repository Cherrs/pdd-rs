use crate::Request;

use serde::{Deserialize, Serialize};


/// 删除商品接口,只能删除下架的商品
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDeleteGoodsCommit {
    
    /// 商品id 列表(List<Long>) json string，例：[1,2]，一次操作数量请小于50
    #[serde(rename = "goods_ids")]
    pub goods_ids: Option<Vec<i64>>,
    
}


/// 删除商品接口,只能删除下架的商品
impl Request for PddDeleteGoodsCommit {
    fn get_type() -> String {
        "pdd.delete.goods.commit".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
