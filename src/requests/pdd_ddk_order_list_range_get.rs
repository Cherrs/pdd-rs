use crate::Request;

use serde::{Deserialize, Serialize};


/// 用时间段查询推广订单接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkOrderListRangeGet {
    
    /// 是否为礼金订单，查询礼金订单时，订单类型不填（默认推广订单）。
    #[serde(rename = "cash_gift_order")]
    pub cash_gift_order: Option<bool>,
    
    /// 支付结束时间，格式: "yyyy-MM-dd HH:mm:ss" ，比如 "2020-12-01 00:00:00"
    #[serde(rename = "end_time")]
    pub end_time: Option<String>,
    
    /// 上一次的迭代器id(第一次不填)
    #[serde(rename = "last_order_id")]
    pub last_order_id: Option<String>,
    
    /// 每次请求多少条，建议300
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 订单类型：1-推广订单；2-直播间订单
    #[serde(rename = "query_order_type")]
    pub query_order_type: Option<i32>,
    
    /// 支付起始时间，格式: "yyyy-MM-dd HH:mm:ss" ，比如 "2020-12-01 00:00:00"
    #[serde(rename = "start_time")]
    pub start_time: Option<String>,
    
}


impl Request for PddDdkOrderListRangeGet {
    fn get_type() -> String {
        "pdd.ddk.order.list.range.get".to_string()
    }

    fn get_response_name() -> String {
        "order_list_get_response".to_string()
    }
}
