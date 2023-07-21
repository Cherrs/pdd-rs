use crate::Request;

use serde::{Deserialize, Serialize};


/// 创建无门槛商品劵批次接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPromotionGoodsCouponCreate {
    
    /// 描述
    #[serde(rename = "batch_desc")]
    pub batch_desc: Option<String>,
    
    /// 开始时间，指到格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)的总毫秒数
    #[serde(rename = "batch_start_time")]
    pub batch_start_time: Option<i64>,
    
    /// 结束时间，指到格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)的总毫秒数
    #[serde(rename = "batch_end_time")]
    pub batch_end_time: Option<i64>,
    
    /// 优惠金额单位: 分
    #[serde(rename = "discount")]
    pub discount: Option<i64>,
    
    /// 可领取数量
    #[serde(rename = "init_quantity")]
    pub init_quantity: Option<i64>,
    
    /// 每个用户限领张数
    #[serde(rename = "user_limit")]
    pub user_limit: Option<i64>,
    
    /// 商品ID
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddPromotionGoodsCouponCreate {
    fn get_type() -> String {
        "pdd.promotion.goods.coupon.create".to_string()
    }

    fn get_response_name() -> String {
        "goods_coupon_batch_create_response".to_string()
    }
}
