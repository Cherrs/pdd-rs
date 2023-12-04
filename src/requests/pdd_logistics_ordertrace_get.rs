use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询单个运单详情
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsOrdertraceGet {
    
    /// 1
    #[serde(rename = "company_code")]
    pub company_code: Option<String>,
    
    /// 1
    #[serde(rename = "mail_no")]
    pub mail_no: Option<String>,
    
}


/// 查询单个运单详情
impl Request for PddLogisticsOrdertraceGet {
    fn get_type() -> String {
        "pdd.logistics.ordertrace.get".to_string()
    }

    fn get_response_name() -> String {
        "logistics_ordertrace_get_resposne".to_string()
    }
}
