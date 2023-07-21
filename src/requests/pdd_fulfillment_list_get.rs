use crate::Request;

use serde::{Deserialize, Serialize};


/// 根据成交时间查询跨境全托管发货单列表（只能获取到成交时间三个月以内的交易信息）①. 一次请求最多能查询时间跨度为24小时的交易记录。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFulfillmentListGet {
    
    /// 成交时间终止（秒）
    #[serde(rename = "end_confirm_at")]
    pub end_confirm_at: Option<i32>,
    
    /// 跨境全托管发货单状态。0-全部，1-待发货，2-已发货待签收，3-已签收
    #[serde(rename = "fulfillment_status")]
    pub fulfillment_status: Option<i32>,
    
    /// 页码
    #[serde(rename = "page_number")]
    pub page_number: Option<i32>,
    
    /// 页面大小
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 成交时间起始（秒）
    #[serde(rename = "start_confirm_at")]
    pub start_confirm_at: Option<i32>,
    
    /// 是否启用has_next的分页方式，默认为true。如果指定true，则返回的结果中不包含总记录数，但是会新增一个是否存在下一页的的字段。
    #[serde(rename = "use_has_next")]
    pub use_has_next: Option<bool>,
    
}


impl Request for PddFulfillmentListGet {
    fn get_type() -> String {
        "pdd.fulfillment.list.get".to_string()
    }

    fn get_response_name() -> String {
        "list_response".to_string()
    }
}
