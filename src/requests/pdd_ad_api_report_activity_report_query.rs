use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询活动报表信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiReportActivityReportQuery {
    
    /// 结束日期的字符串，格式类似'2020-02-02'
    #[serde(rename = "endDateString")]
    pub end_date_string: Option<String>,
    
    /// 场景类型：3联合推广。
    #[serde(rename = "scenesType")]
    pub scenes_type: Option<i32>,
    
    /// 开始日期的字符串，格式类似'2020-02-02'，如果查询今日，startDateString和endDateString传今日的字符串，如果查询历史，startDateString和endDateString分别传开始和结束字符串，不能跨今日和历史查询
    #[serde(rename = "startDateString")]
    pub start_date_string: Option<String>,
    
}


impl Request for PddAdApiReportActivityReportQuery {
    fn get_type() -> String {
        "pdd.ad.api.report.activity.report.query".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
