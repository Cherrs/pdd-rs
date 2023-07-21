use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询全站推广小时报表数据
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiReportTrHourlyReportQuery {
    
    /// 查询日期的字符串，格式类似'2020-02-02',当前支持查询30天内数据
    #[serde(rename = "dateString")]
    pub date_string: Option<String>,
    
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddAdApiReportTrHourlyReportQuery {
    fn get_type() -> String {
        "pdd.ad.api.report.tr.hourly.report.query".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
