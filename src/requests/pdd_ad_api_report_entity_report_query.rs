use crate::Request;

use serde::{Deserialize, Serialize};


/// 用于查询广告主报表某个实体下子级实体报表数据，返回的结果按查询的子实体维度分组，每条记录为子实体一段时间的汇总数据，例如查询广告主（查询实体）下所有计划（子实体）某一时段的报表信息，返回的记录按计划分组，每条记录为每个计划这段时间的汇总报表数据。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct QueryRange {
    
    /// 页数
    #[serde(rename = "pageNumber")]
    pub page_number: Option<i32>,
    
    /// 每页的数量
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    
}

/// 用于查询广告主报表某个实体下子级实体报表数据，返回的结果按查询的子实体维度分组，每条记录为子实体一段时间的汇总数据，例如查询广告主（查询实体）下所有计划（子实体）某一时段的报表信息，返回的记录按计划分组，每条记录为每个计划这段时间的汇总报表数据。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExternalParamMap {
    
    /// key
    #[serde(rename = "$key")]
    pub key: Option<String>,
    
    /// value
    #[serde(rename = "$value")]
    pub value: Option<String>,
    
}

/// 用于查询广告主报表某个实体下子级实体报表数据，返回的结果按查询的子实体维度分组，每条记录为子实体一段时间的汇总数据，例如查询广告主（查询实体）下所有计划（子实体）某一时段的报表信息，返回的记录按计划分组，每条记录为每个计划这段时间的汇总报表数据。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiReportEntityReportQuery {
    
    /// 结束日期的字符串，格式类似'2020-02-02'，当前支持查询90天内数据
    #[serde(rename = "endDateString")]
    pub end_date_string: Option<String>,
    
    /// entityId的维度(当前只支持0-广告主，1-计划，2-单元维度),例如根据单元查询资源位的分级数据，entityId传单元id，entityDimensionType传单元维度，queryDimensionType传资源位维度
    #[serde(rename = "entityDimensionType")]
    pub entity_dimension_type: Option<i32>,
    
    /// 各维度查询的主体id，查询计划维度传计划id，查询单元维度传单元id
    #[serde(rename = "entityId")]
    pub entity_id: Option<i64>,
    
    /// 额外的查询条件，entityDimensionType维度为单元时，须加上父级计划id（planId）的信息
    #[serde(rename = "externalParamMap")]
    pub external_param_map: Option<ExternalParamMap>,
    
    /// 排序规则，0-曝光，1-点击，2-点击率，3-cpc,4-花费，5-订单量,6-gmv，7-roi,8-日期，9-cpm,10-店铺收藏，11-商品收藏
    #[serde(rename = "orderBy")]
    pub order_by: Option<i32>,
    
    /// 排序顺序，0-降序，1-升序
    #[serde(rename = "orderType")]
    pub order_type: Option<i32>,
    
    /// 查询维度，0-广告主，1-计划，2-单元，3-定向，4-创意，5-资源位，6-关键词
    #[serde(rename = "queryDimensionType")]
    pub query_dimension_type: Option<i32>,
    
    /// 分页字段,不传不分页
    #[serde(rename = "queryRange")]
    pub query_range: Option<QueryRange>,
    
    /// 场景类型。0表示搜索，1明星店铺，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: Option<i32>,
    
    /// 开始日期的字符串，格式类似'2020-02-02'，如果查询今日，startDateString和endDateString传今日的字符串，如果查询历史，startDateString和endDateString分别传开始和结束字符串，不能跨今日和历史查询
    #[serde(rename = "startDateString")]
    pub start_date_string: Option<String>,
    
}


impl Request for PddAdApiReportEntityReportQuery {
    fn get_type() -> String {
        "pdd.ad.api.report.entity.report.query".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
