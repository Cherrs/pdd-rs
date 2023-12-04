use crate::Request;

use serde::{Deserialize, Serialize};


/// 用于ISV查询自己名下的服务的交易明细单
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddServicemarketTradelistGet {
    
    /// 查询起始时间，精确到秒，起止时间间隔最大31天
    #[serde(rename = "begin_time")]
    pub begin_time: Option<i32>,
    
    /// 查询结束时间，精确到秒，起止时间间隔最大31天
    #[serde(rename = "end_time")]
    pub end_time: Option<i32>,
    
    /// 收支类型，空-全部 1-收入 2-支出
    #[serde(rename = "group_type")]
    pub group_type: Option<i32>,
    
    /// 分页页码，最大1000
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 分页大小，最大100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 服务订单号
    #[serde(rename = "service_order_sn")]
    pub service_order_sn: Option<String>,
    
}


/// 用于ISV查询自己名下的服务的交易明细单
impl Request for PddServicemarketTradelistGet {
    fn get_type() -> String {
        "pdd.servicemarket.tradelist.get".to_string()
    }

    fn get_response_name() -> String {
        "mall_balance_flow_search_response".to_string()
    }
}
