use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询广告主的各维度的分天报表，当前支持广告主，计划，单元，创意，关键词，定向，资源位等维度，当前仅支持单实体的查询，不支持批量实体的查询，返回的结果按天分组
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExternalParamMap {
    
    /// key
    #[serde(rename = "$key")]
    pub key: Option<String>,
    
    /// value
    #[serde(rename = "$value")]
    pub value: Option<String>,
    
}

/// 查询广告主的各维度的分天报表，当前支持广告主，计划，单元，创意，关键词，定向，资源位等维度，当前仅支持单实体的查询，不支持批量实体的查询，返回的结果按天分组
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiReportDailyReportQuery {
    
    /// 结束日期的字符串，格式类似'2020-02-02'，当前支持查询90天内数据
    #[serde(rename = "endDateString")]
    pub end_date_string: Option<String>,
    
    /// 各维度查询的主体id，查询计划维度传计划id，查询单元维度传单元id，查询关键词维度传关键词id，查询创意传创意id，查询广告主维度，资源位和定向维度不用传此参数
    #[serde(rename = "entityId")]
    pub entity_id: Option<i64>,
    
    /// 额外的查询条件，查询关键词，创意维度的的时候要在此传单元id(adId)的信息，查询资源位的时候要传单元id(adId）和资源位类型(bidReferenceId),查询定向维度要传单元id(adId),定向类型(bidReferenceId)，如果是查二级定向，需要传二级定向id(subBidReferenceId)
    #[serde(rename = "externalParamMap")]
    pub external_param_map: Option<ExternalParamMap>,
    
    /// 查询维度，0-广告主，1-计划，2-单元，3-定向，4-创意，5-资源位，6-关键词
    #[serde(rename = "queryDimensionType")]
    pub query_dimension_type: Option<i32>,
    
    /// 场景类型。0表示搜索，1明星店铺，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: Option<i32>,
    
    /// 开始日期的字符串，格式类似'2020-02-02'，如果查询今日，startDateString和endDateString传今日的字符串，如果查询历史，startDateString和endDateString分别传开始和结束字符串，不能跨今日和历史查询
    #[serde(rename = "startDateString")]
    pub start_date_string: Option<String>,
    
}


/// 查询广告主的各维度的分天报表，当前支持广告主，计划，单元，创意，关键词，定向，资源位等维度，当前仅支持单实体的查询，不支持批量实体的查询，返回的结果按天分组
impl Request for PddAdApiReportDailyReportQuery {
    fn get_type() -> String {
        "pdd.ad.api.report.daily.report.query".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
