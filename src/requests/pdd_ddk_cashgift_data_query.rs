use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询多多礼金效果数据
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkCashgiftDataQuery {
    
    /// 礼金ID，支持根据礼金ID查询
    #[serde(rename = "cash_gift_id")]
    pub cash_gift_id: Option<i64>,
    
    /// 礼金创建结束时间，查询该时间段内创建的所有礼金效果数据（礼金维度）。note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    #[serde(rename = "end_time")]
    pub end_time: Option<i64>,
    
    /// 分页数
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 每页结果数
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 礼金创建起始时间，查询该时间段内创建的所有礼金效果数据（礼金维度）。note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    #[serde(rename = "start_time")]
    pub start_time: Option<i64>,
    
}


impl Request for PddDdkCashgiftDataQuery {
    fn get_type() -> String {
        "pdd.ddk.cashgift.data.query".to_string()
    }

    fn get_response_name() -> String {
        "cashgift_data_response".to_string()
    }
}
