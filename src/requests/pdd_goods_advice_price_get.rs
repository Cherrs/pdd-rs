use crate::Request;

use serde::{Deserialize, Serialize};


/// 商家可通过此接口查询店铺内所有上架商品的建议价格
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsAdvicePriceGet {
    
    /// 获取商品建议价格请求参数
    #[serde(rename = "request")]
    pub request: Option<PddGoodsAdvicePriceGetRequest>,
    
}

/// 商家可通过此接口查询店铺内所有上架商品的建议价格
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsAdvicePriceGetRequest {
    
    /// 页码，默认1
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 每页数量，默认100，最大100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
}


impl Request for PddGoodsAdvicePriceGet {
    fn get_type() -> String {
        "pdd.goods.advice.price.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_advice_price_get_response".to_string()
    }
}
