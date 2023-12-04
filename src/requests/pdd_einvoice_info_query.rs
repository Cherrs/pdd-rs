use crate::Request;

use serde::{Deserialize, Serialize};


/// 商家使用自动开票系统对订单进行开票，可通过此接口获取30天内已开发票对应的发票和订单信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddEinvoiceInfoQuery {
    
    /// 最后更新时间结束时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数。开始时间结束时间间距不超过1小时。不能查询最近5分钟内的数据。开区间，返回数据不包含end_time
    #[serde(rename = "end_time")]
    pub end_time: Option<i64>,
    
    /// 发票类型 0-蓝票，1-红票，不传为全部
    #[serde(rename = "invoice_type")]
    pub invoice_type: Option<i32>,
    
    /// 页码。页码从 1开始
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 返回数量。最小1，最大 50
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 最后更新时间开始时间的时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数。只能查询30天内的数据。闭区间，返回数据包含start_time
    #[serde(rename = "start_time")]
    pub start_time: Option<i64>,
    
}


/// 商家使用自动开票系统对订单进行开票，可通过此接口获取30天内已开发票对应的发票和订单信息
impl Request for PddEinvoiceInfoQuery {
    fn get_type() -> String {
        "pdd.einvoice.info.query".to_string()
    }

    fn get_response_name() -> String {
        "einvoice_info_query_response".to_string()
    }
}
