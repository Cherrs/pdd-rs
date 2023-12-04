use crate::Request;

use serde::{Deserialize, Serialize};


/// 修改商品sku价格
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSkuPriceUpdate {
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 是否获取商品发布警告信息，默认为忽略
    #[serde(rename = "ignore_edit_warn")]
    pub ignore_edit_warn: Option<bool>,
    
    /// 参考价 （单位分）
    #[serde(rename = "market_price")]
    pub market_price: Option<i64>,
    
    /// 参考价 （单位元）
    #[serde(rename = "market_price_in_yuan")]
    pub market_price_in_yuan: Option<String>,
    
    /// 待修改的sku价格
    #[serde(rename = "sku_price_list")]
    pub sku_price_list: Option<Vec<SkuPriceList>>,
    
    /// 提交后上架状态，0:上架,1:保持原样
    #[serde(rename = "sync_goods_operate")]
    pub sync_goods_operate: Option<i32>,
    
    /// 满2件折扣，可选范围0-100, 0表示取消，95表示95折，设置需先查询规则接口获取实际可填范围
    #[serde(rename = "two_pieces_discount")]
    pub two_pieces_discount: Option<i32>,
    
}

/// 修改商品sku价格
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SkuPriceList {
    
    /// 拼团购买价格（单位分）
    #[serde(rename = "group_price")]
    pub group_price: Option<i64>,
    
    /// sku上架状态，0-已下架，1-上架中
    #[serde(rename = "is_onsale")]
    pub is_onsale: Option<i32>,
    
    /// 单独购买价格（单位分）
    #[serde(rename = "single_price")]
    pub single_price: Option<i64>,
    
    /// sku标识
    #[serde(rename = "sku_id")]
    pub sku_id: Option<i64>,
    
}


/// 修改商品sku价格
impl Request for PddGoodsSkuPriceUpdate {
    fn get_type() -> String {
        "pdd.goods.sku.price.update".to_string()
    }

    fn get_response_name() -> String {
        "goods_update_sku_price_response".to_string()
    }
}
