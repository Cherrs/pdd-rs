use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品关联信息设置
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsRelationSet {
    
    /// 拼多多商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 月销量
    #[serde(rename = "month_sales")]
    pub month_sales: Option<i64>,
    
    /// 外部平台商品url
    #[serde(rename = "out_detail_url")]
    pub out_detail_url: Option<String>,
    
    /// 外部平台商品id
    #[serde(rename = "out_goods_id")]
    pub out_goods_id: Option<String>,
    
    /// 外部平台商品最高价，单位：分
    #[serde(rename = "out_high_goods_price")]
    pub out_high_goods_price: Option<i64>,
    
    /// 外部平台商品最低价，单位：分
    #[serde(rename = "out_low_goods_price")]
    pub out_low_goods_price: Option<i64>,
    
    /// 外部平台店铺id
    #[serde(rename = "out_mall_id")]
    pub out_mall_id: Option<i64>,
    
    /// 邮费
    #[serde(rename = "postage")]
    pub postage: Option<i64>,
    
}


/// 商品关联信息设置
impl Request for PddGoodsRelationSet {
    fn get_type() -> String {
        "pdd.goods.relation.set".to_string()
    }

    fn get_response_name() -> String {
        "goods_relation_set_response".to_string()
    }
}
