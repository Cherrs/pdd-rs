use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询可用商品列表（分页）
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiGoodsQueryPage {
    
    /// 商品名称
    #[serde(rename = "goodsName")]
    pub goods_name: Option<String>,
    
    /// 分页查询，查询第几页
    #[serde(rename = "pageNumber")]
    pub page_number: Option<i32>,
    
    /// 分页查询，每页的大小
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    
    /// 计划Id
    #[serde(rename = "planId")]
    pub plan_id: Option<i64>,
    
}


impl Request for PddAdApiGoodsQueryPage {
    fn get_type() -> String {
        "pdd.ad.api.goods.query.page".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
