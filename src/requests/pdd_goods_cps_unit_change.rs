use crate::Request;

use serde::{Deserialize, Serialize};


/// 修改推广商品API
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCpsUnitChange {
    
    /// 优惠券结束时间
    #[serde(rename = "coupon_end_time")]
    pub coupon_end_time: Option<String>,
    
    /// 优惠券id
    #[serde(rename = "coupon_id")]
    pub coupon_id: Option<i64>,
    
    /// 优惠券号
    #[serde(rename = "coupon_sn")]
    pub coupon_sn: Option<String>,
    
    /// 优惠券开始时间
    #[serde(rename = "coupon_start_time")]
    pub coupon_start_time: Option<String>,
    
    /// 优惠券面额（单位为分）
    #[serde(rename = "discount")]
    pub discount: Option<i32>,
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 设置的优惠券张数
    #[serde(rename = "init_quantity")]
    pub init_quantity: Option<i64>,
    
    /// 佣金比例（千分比）
    #[serde(rename = "rate")]
    pub rate: Option<i32>,
    
    /// 优惠券剩余数量
    #[serde(rename = "remain_quantity")]
    pub remain_quantity: Option<i64>,
    
    /// 优惠券领取后的有效使用时间天数
    #[serde(rename = "duration")]
    pub duration: Option<i32>,
    
}


/// 修改推广商品API
impl Request for PddGoodsCpsUnitChange {
    fn get_type() -> String {
        "pdd.goods.cps.unit.change".to_string()
    }

    fn get_response_name() -> String {
        "is_change_success".to_string()
    }
}
