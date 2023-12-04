use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品列表查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsListGet {
    
    /// 模版id
    #[serde(rename = "cost_template_id")]
    pub cost_template_id: Option<i64>,
    
    /// 商品创建时间结束时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至结束时间的总秒数 PS：开始时间结束时间间距不超过7天
    #[serde(rename = "created_at_end")]
    pub created_at_end: Option<i64>,
    
    /// 商品创建时间开始时间的时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至开始时间的总秒数
    #[serde(rename = "created_at_from")]
    pub created_at_from: Option<i64>,
    
    /// 商品名称模糊查询,outer_id,is_onsale,goods_name三选一，优先级is_onsale>outer_id>goods_name
    #[serde(rename = "goods_name")]
    pub goods_name: Option<String>,
    
    /// 上下架状态，0-下架，1-上架,outer_id,is_onsale,goods_name三选一，优先级is_onsale>outer_id>goods_name
    #[serde(rename = "is_onsale")]
    pub is_onsale: Option<i32>,
    
    /// 商家外部商品编码，支持多个，用逗号隔开，最多10个
    #[serde(rename = "outer_goods_id")]
    pub outer_goods_id: Option<String>,
    
    /// 商品外部编码（sku），同其他接口中的outer_id 、out_id、out_sku_sn、outer_sku_sn、out_sku_id、outer_sku_id 都为商家编码（sku维度）。outer_id,is_onsale,goods_name三选一，优先级is_onsale>outer_id>goods_name
    #[serde(rename = "outer_id")]
    pub outer_id: Option<String>,
    
    /// 返回页码 默认 1，页码从 1 开始PS：当前采用分页返回，数量和页数会一起传，如果不传，则采用 默认值
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 返回数量，默认 100，最大100。
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
}


/// 商品列表查询
impl Request for PddGoodsListGet {
    fn get_type() -> String {
        "pdd.goods.list.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_list_get_response".to_string()
    }
}
