use crate::Request;

use serde::{Deserialize, Serialize};


/// 快递公司工单查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsTicketGet {
    
    /// 必填，最后更新时间结束时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数 PS：开始时间结束时间间距不超过 30 分钟。示例：1523763012。
    #[serde(rename = "end_updated_at")]
    pub end_updated_at: Option<i64>,
    
    /// 返回页码 默认 1，页码从 1 开始 PS：当前采用分页返回，数量和页数会一起传，如果不传，则采用 默认值。注：必须采用倒序的分页方式（从最后一页往回取）才能避免漏单问题
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 返回数量，默认 100。最大 100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 必填，最后更新时间开始时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数。示例：1523762012。
    #[serde(rename = "start_updated_at")]
    pub start_updated_at: Option<i64>,
    
}


/// 快递公司工单查询
impl Request for PddLogisticsTicketGet {
    fn get_type() -> String {
        "pdd.logistics.ticket.get".to_string()
    }

    fn get_response_name() -> String {
        "logistics_ticket_get_response".to_string()
    }
}
