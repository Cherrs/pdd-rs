use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询面单服务订购及面单使用情况
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddWaybillSearch {
    
    /// 物流公司code
    #[serde(rename = "wp_code")]
    pub wp_code: Option<String>,
    
}


/// 查询面单服务订购及面单使用情况
impl Request for PddWaybillSearch {
    fn get_type() -> String {
        "pdd.waybill.search".to_string()
    }

    fn get_response_name() -> String {
        "pdd_waybill_search_response".to_string()
    }
}
