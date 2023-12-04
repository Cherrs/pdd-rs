use crate::Request;

use serde::{Deserialize, Serialize};


/// 关闭批次接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPromotionCouponClose {
    
    /// 券批次ID
    #[serde(rename = "batch_id")]
    pub batch_id: Option<i64>,
    
}


/// 关闭批次接口
impl Request for PddPromotionCouponClose {
    fn get_type() -> String {
        "pdd.promotion.coupon.close".to_string()
    }

    fn get_response_name() -> String {
        "promotion_coupon_batch_close_response".to_string()
    }
}
