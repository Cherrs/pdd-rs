use crate::Request;

use serde::{Deserialize, Serialize};


/// 厂家首次接入ISV时，同步商家分配给厂家历史订单列表，最多支持同步近一个月数据
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFdsOrderListGet {
    
    /// 入参信息
    #[serde(rename = "param_fds_order_list_get_request")]
    pub param_fds_order_list_get_request: Option<ParamFdsOrderListGetRequest>,
    
}

/// 厂家首次接入ISV时，同步商家分配给厂家历史订单列表，最多支持同步近一个月数据
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParamFdsOrderListGetRequest {
    
    /// 必填，更新时间结束时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总毫秒数 PS：开始时间结束时间间距不超过半小时
    #[serde(rename = "end_updated_time")]
    pub end_updated_time: Option<i64>,
    
    /// 返回页码，页码从 1 开始 PS：当前采用分页返回，数量和页数会一起传
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 返回数量，最大 100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 必填，更新时间开始时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总毫秒数 PS：开始时间结束时间间距不超过半小时
    #[serde(rename = "start_updated_time")]
    pub start_updated_time: Option<i64>,
    
}


/// 厂家首次接入ISV时，同步商家分配给厂家历史订单列表，最多支持同步近一个月数据
impl Request for PddFdsOrderListGet {
    fn get_type() -> String {
        "pdd.fds.order.list.get".to_string()
    }

    fn get_response_name() -> String {
        "pdd_fds_order_list_get_response".to_string()
    }
}
