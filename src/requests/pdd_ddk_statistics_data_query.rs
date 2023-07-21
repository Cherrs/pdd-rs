use crate::Request;

use serde::{Deserialize, Serialize};


/// 本接口用于查询特定活动数据，仅限特定渠道可用。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkStatisticsDataQuery {
    
    /// 分页数，默认值: 1
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 每页结果数，默认值: 20
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 周期类型: 1-每7天，2-自然月
    #[serde(rename = "period_type")]
    pub period_type: Option<i32>,
    
    /// 数据类型: 1-增量补贴数据
    #[serde(rename = "statistics_type")]
    pub statistics_type: Option<i32>,
    
    /// 查询时间点，格式: "yyyy-MM-dd"。period_type为1时，查询时间点前7天的数据；period_type为2时，查询时间点所在自然月的数据。
    #[serde(rename = "time")]
    pub time: Option<String>,
    
}


impl Request for PddDdkStatisticsDataQuery {
    fn get_type() -> String {
        "pdd.ddk.statistics.data.query".to_string()
    }

    fn get_response_name() -> String {
        "statistics_data_response".to_string()
    }
}
