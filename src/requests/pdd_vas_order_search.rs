use crate::Request;

use serde::{Deserialize, Serialize};


/// 用于拉取回流完成的订单以及线上增量的订购订单
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVasOrderSearch {
    
    /// 创建时间结束，UNIX时间戳（ms 级别），默认为当前时间，支持最大范围为7天。
    #[serde(rename = "create_time_end")]
    pub create_time_end: Option<i64>,
    
    /// 创建时间开始，UNIX时间戳（ms级别），默认为当前时间往前推7天，支持最大范围为7天。
    #[serde(rename = "create_time_start")]
    pub create_time_start: Option<i64>,
    
    /// 买家店铺id
    #[serde(rename = "mall_id")]
    pub mall_id: Option<i64>,
    
    /// 服务订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 订单状态，枚举值，0-未完成，1-已完成，2-已取消，空-全部
    #[serde(rename = "order_status")]
    pub order_status: Option<i32>,
    
    /// 分页页码
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 分页大小，最大支持100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 支付时间开始，UNIX时间戳（ms 级别）
    #[serde(rename = "pay_time_end")]
    pub pay_time_end: Option<i64>,
    
    /// 支付时间开始，UNIX时间戳（ms 级别）
    #[serde(rename = "pay_time_start")]
    pub pay_time_start: Option<i64>,
    
    /// 服务sku_id，可在服务详情页中获取
    #[serde(rename = "sku_id")]
    pub sku_id: Option<i64>,
    
    /// 售后状态，0-未售后，1-已售后
    #[serde(rename = "refund_status")]
    pub refund_status: Option<i32>,
    
}


/// 用于拉取回流完成的订单以及线上增量的订购订单
impl Request for PddVasOrderSearch {
    fn get_type() -> String {
        "pdd.vas.order.search".to_string()
    }

    fn get_response_name() -> String {
        "vas_order_search_response".to_string()
    }
}
