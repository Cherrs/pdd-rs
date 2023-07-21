use crate::Request;

use serde::{Deserialize, Serialize};


/// 结束已创建的限时限量购活动
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPromotionLimitedActivityCancel {
    
    /// 活动id
    #[serde(rename = "detail_id")]
    pub detail_id: Option<i64>,
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddPromotionLimitedActivityCancel {
    fn get_type() -> String {
        "pdd.promotion.limited.activity.cancel".to_string()
    }

    fn get_response_name() -> String {
        "result".to_string()
    }
}
