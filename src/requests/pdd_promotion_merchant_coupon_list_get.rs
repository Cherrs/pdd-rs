use crate::Request;

use serde::{Deserialize, Serialize};


/// 店铺优惠券批次列表接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPromotionMerchantCouponListGet {
    
    /// 页码，默认1
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 每页数量，默认100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 批次开始时间（范围开始）
    #[serde(rename = "batch_start_time_from")]
    pub batch_start_time_from: Option<i64>,
    
    /// 批次开始时间（范围结束）
    #[serde(rename = "batch_start_time_to")]
    pub batch_start_time_to: Option<i64>,
    
    /// 批次状态1 领取中，2 已领完，3 已结束
    #[serde(rename = "batch_status")]
    pub batch_status: Option<i32>,
    
    /// 排序1 创建时间正序，2 创建时间倒序，3 开始时间正序，4 开始时间倒序，5 初始数量正序， 6 初始数量倒序，7 领取数量正序，8 领取数量倒序；默认2
    #[serde(rename = "sort_by")]
    pub sort_by: Option<i32>,
    
}


impl Request for PddPromotionMerchantCouponListGet {
    fn get_type() -> String {
        "pdd.promotion.merchant.coupon.list.get".to_string()
    }

    fn get_response_name() -> String {
        "merchant_coupon_batch_list_response".to_string()
    }
}
