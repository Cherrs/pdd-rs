use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询全站推广分天报表数据
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiReportTrDailyReportQuery {
    
    /// 结束日期的字符串，格式类似'2020-02-02'
    #[serde(rename = "endDateString")]
    pub end_date_string: Option<String>,
    
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
    /// 开始日期的字符串，格式类似'2020-02-02'，如果查询今日，startDateString和endDateString传今日的字符串，如果查询历史，startDateString和endDateString分别传开始和结束字符串，不能跨今日和历史查询,,当前支持查询90天内数据
    #[serde(rename = "startDateString")]
    pub start_date_string: Option<String>,
    
}


/// 查询全站推广分天报表数据
impl Request for PddAdApiReportTrDailyReportQuery {
    fn get_type() -> String {
        "pdd.ad.api.report.tr.daily.report.query".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
