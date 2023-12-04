use crate::Request;

use serde::{Deserialize, Serialize};


/// 家电分仓库存-根据商品id查询sku信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockGoodsIdToSkuQuery {
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 是否需要查询下架商品和sku，默认不需要
    #[serde(rename = "need_offsale")]
    pub need_offsale: Option<bool>,
    
    /// 货品id
    #[serde(rename = "ware_id")]
    pub ware_id: Option<i64>,
    
}


/// 家电分仓库存-根据商品id查询sku信息
impl Request for PddStockGoodsIdToSkuQuery {
    fn get_type() -> String {
        "pdd.stock.goods.id.to.sku.query".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
