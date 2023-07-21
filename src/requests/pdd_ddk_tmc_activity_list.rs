use crate::Request;

use serde::{Deserialize, Serialize};


/// 支持当前日期前6天到后7天的时间范围查询千万神券活动，日期超过范围将用最大边界时间替换
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkTmcActivityList {
    
    /// 页码 从1开始
    #[serde(rename = "page_num")]
    pub page_num: Option<i32>,
    
    /// 每页结果数，默认值: 20 最大50
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 活动开始时间最小时间 格式: "yyyy-MM-dd HH:mm:ss"
    #[serde(rename = "start_time_lower")]
    pub start_time_lower: Option<String>,
    
    /// 活动开始时间最大时间 格式: "yyyy-MM-dd HH:mm:ss"
    #[serde(rename = "start_time_upper")]
    pub start_time_upper: Option<String>,
    
}


impl Request for PddDdkTmcActivityList {
    fn get_type() -> String {
        "pdd.ddk.tmc.activity.list".to_string()
    }

    fn get_response_name() -> String {
        "tmc_aty_list_response".to_string()
    }
}
