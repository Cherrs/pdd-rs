use crate::Request;

use serde::{Deserialize, Serialize};


/// 按照时间段获取授权多多客下面所有多多客的推广订单信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkAllOrderListIncrementGet {
    
    /// 查询结束时间，和开始时间相差不能超过24小时。note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    #[serde(rename = "end_update_time")]
    pub end_update_time: Option<i64>,
    
    /// 第几页，从1到10000，默认1，注：使用最后更新时间范围增量同步时，必须采用倒序的分页方式（从最后一页往回取）才能避免漏单问题。
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 返回的每页结果订单数，默认为100，范围为10到100，建议使用40~50，可以提高成功率，减少超时数量。
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 订单类型：1-推广订单；2-直播间订单
    #[serde(rename = "query_order_type")]
    pub query_order_type: Option<i32>,
    
    /// 最近90天内多多进宝商品订单更新时间--查询时间开始。note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    #[serde(rename = "start_update_time")]
    pub start_update_time: Option<i64>,
    
}


impl Request for PddDdkAllOrderListIncrementGet {
    fn get_type() -> String {
        "pdd.ddk.all.order.list.increment.get".to_string()
    }

    fn get_response_name() -> String {
        "order_list_get_response".to_string()
    }
}
