use crate::Request;

use serde::{Deserialize, Serialize};


/// 根据成交时间查询订单列表（只能获取到成交时间三个月以内的交易信息）注：虚拟订单充值手机号信息无法通过此接口获取，请联系虚拟类目运营人员。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOrderListGet {
    
    /// 必填，成交时间结束时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数 PS：开始时间结束时间间距不超过 24 小时
    #[serde(rename = "end_confirm_at")]
    pub end_confirm_at: Option<i64>,
    
    /// 发货状态，1：待发货，2：已发货待签收，3：已签收 5：全部
    #[serde(rename = "order_status")]
    pub order_status: Option<i32>,
    
    /// 返回页码 默认 1，页码从 1 开始 PS：当前采用分页返回，数量和页数会一起传，如果不传，则采用 默认值
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 返回数量，默认 100。最大 100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 售后状态 1：无售后或售后关闭，2：售后处理中，3：退款中，4： 退款成功 5：全部
    #[serde(rename = "refund_status")]
    pub refund_status: Option<i32>,
    
    /// 必填，成交时间开始时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    #[serde(rename = "start_confirm_at")]
    pub start_confirm_at: Option<i64>,
    
    /// 订单类型 0-普通订单 ，1- 定金订单
    #[serde(rename = "trade_type")]
    pub trade_type: Option<i32>,
    
    /// 是否启用has_next的分页方式，如果指定true,则返回的结果中不包含总记录数，但是会新增一个是否存在下一页的的字段，通过此种方式获取增量交易，效率在原有的基础上有80%的提升。
    #[serde(rename = "use_has_next")]
    pub use_has_next: Option<bool>,
    
}


impl Request for PddOrderListGet {
    fn get_type() -> String {
        "pdd.order.list.get".to_string()
    }

    fn get_response_name() -> String {
        "order_list_get_response".to_string()
    }
}
