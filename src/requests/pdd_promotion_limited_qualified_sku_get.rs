use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询可参加限时限量购活动的sku列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPromotionLimitedQualifiedSkuGet {
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


/// 查询可参加限时限量购活动的sku列表
impl Request for PddPromotionLimitedQualifiedSkuGet {
    fn get_type() -> String {
        "pdd.promotion.limited.qualified.sku.get".to_string()
    }

    fn get_response_name() -> String {
        "in_valid_sku_List".to_string()
    }
}
