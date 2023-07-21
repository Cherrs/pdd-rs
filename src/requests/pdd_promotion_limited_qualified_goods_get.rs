use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询可参与限时限量购活动的商品
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPromotionLimitedQualifiedGoodsGet {
    
    /// 商品id列表
    #[serde(rename = "goods_id_list")]
    pub goods_id_list: Option<Vec<i64>>,
    
    /// TRUE-仅查询可选商品（满足活动资格商品）数据；FALSE-查询不可选商品数据
    #[serde(rename = "is_valid")]
    pub is_valid: Option<bool>,
    
    /// 页码
    #[serde(rename = "page_no")]
    pub page_no: Option<i32>,
    
    /// 每页查询数
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
}


impl Request for PddPromotionLimitedQualifiedGoodsGet {
    fn get_type() -> String {
        "pdd.promotion.limited.qualified.goods.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_list".to_string()
    }
}
