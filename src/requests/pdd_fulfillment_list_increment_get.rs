use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询跨境全托管发货单增量。（只能获取到成交时间三个月以内的交易信息）①. 一次请求只能查询时间跨度为30分钟的增量交易记录，即end_updated_at - start_updated_at<= 30min。②. 通过从后往前翻页的方式以及每次获取的起始时间适当前移10分钟左右可以避免漏单问题。如当前时间12:33，拉取12:00~12:30，下次拉取范围建议12:20~12:50。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFulfillmentListIncrementGet {
    
    /// 更新时间终止（秒）
    #[serde(rename = "end_update_at")]
    pub end_update_at: Option<i32>,
    
    /// 跨境全托管发货单状态。0-全部，1-待发货，2-已发货待签收，3-已签收
    #[serde(rename = "fulfillment_status")]
    pub fulfillment_status: Option<i32>,
    
    /// 页码
    #[serde(rename = "page_number")]
    pub page_number: Option<i32>,
    
    /// 页面大小
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 更新时间起始（秒）
    #[serde(rename = "start_update_at")]
    pub start_update_at: Option<i32>,
    
    /// 是否启用has_next的分页方式，默认为true。如果指定true，则返回的结果中不包含总记录数，但是会新增一个是否存在下一页的的字段。
    #[serde(rename = "use_has_next")]
    pub use_has_next: Option<bool>,
    
}


impl Request for PddFulfillmentListIncrementGet {
    fn get_type() -> String {
        "pdd.fulfillment.list.increment.get".to_string()
    }

    fn get_response_name() -> String {
        "list_response".to_string()
    }
}
