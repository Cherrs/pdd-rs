use crate::Request;

use serde::{Deserialize, Serialize};


/// 修改商品sku库存，在资源位上的商品不能减少库存
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsQuantityUpdate {
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 库存修改值。当全量更新库存时，quantity必须为大于等于0的正整数；当增量更新库存时，quantity为整数，可小于等于0。若增量更新时传入的库存为负数，则负数与实际库存之和不能小于0。比如当前实际库存为1，传入增量更新quantity=-1，库存改为0
    #[serde(rename = "quantity")]
    pub quantity: Option<i64>,
    
    /// sku_id和outer_id必填一个，优先使用sku_id
    #[serde(rename = "sku_id")]
    pub sku_id: Option<i64>,
    
    /// sku商家编码，如果sku_id未填，则使用outer_id
    #[serde(rename = "outer_id")]
    pub outer_id: Option<String>,
    
    /// 库存更新方式，可选。1为全量更新，2为增量更新。如果不填，默认为全量更新
    #[serde(rename = "update_type")]
    pub update_type: Option<i32>,
    
}


impl Request for PddGoodsQuantityUpdate {
    fn get_type() -> String {
        "pdd.goods.quantity.update".to_string()
    }

    fn get_response_name() -> String {
        "goods_quantity_update_response".to_string()
    }
}
