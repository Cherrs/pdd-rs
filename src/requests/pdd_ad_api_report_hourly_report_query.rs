use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询推广报表各维度（广告主，计划，单元）的小时报表数据
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiReportHourlyReportQuery {
    
    /// 查询日期的字符串，格式类似'2020-02-02',当前支持查询30天内数据
    #[serde(rename = "dateString")]
    pub date_string: Option<String>,
    
    /// 各维度查询的主体id，查询计划维度传计划id，查询单元维度传单元id
    #[serde(rename = "entityId")]
    pub entity_id: Option<i64>,
    
    /// 查询维度，0-广告主，1-计划，2-单元,当前只支持到单元维度
    #[serde(rename = "queryDimensionType")]
    pub query_dimension_type: Option<i32>,
    
    /// 场景类型。0表示搜索，1明星店铺，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: Option<i32>,
    
}


/// 查询推广报表各维度（广告主，计划，单元）的小时报表数据
impl Request for PddAdApiReportHourlyReportQuery {
    fn get_type() -> String {
        "pdd.ad.api.report.hourly.report.query".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
