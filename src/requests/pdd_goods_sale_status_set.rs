use crate::Request;

use serde::{Deserialize, Serialize};


/// 修改商品上下架状态
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSaleStatusSet {
    
    /// 拼多多商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 上下架状态：1:上架 0:下架
    #[serde(rename = "is_onsale")]
    pub is_onsale: Option<i32>,
    
}


/// 修改商品上下架状态
impl Request for PddGoodsSaleStatusSet {
    fn get_type() -> String {
        "pdd.goods.sale.status.set".to_string()
    }

    fn get_response_name() -> String {
        "goods_sale_status_set_response".to_string()
    }
}
