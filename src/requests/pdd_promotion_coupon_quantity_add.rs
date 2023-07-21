use crate::Request;

use serde::{Deserialize, Serialize};


/// 增加优惠券发行数量接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPromotionCouponQuantityAdd {
    
    /// 券批次ID
    #[serde(rename = "batch_id")]
    pub batch_id: Option<i64>,
    
    /// 要增加的数量
    #[serde(rename = "add_quantity")]
    pub add_quantity: Option<i64>,
    
}


impl Request for PddPromotionCouponQuantityAdd {
    fn get_type() -> String {
        "pdd.promotion.coupon.quantity.add".to_string()
    }

    fn get_response_name() -> String {
        "coupon_quantity_add_response".to_string()
    }
}
