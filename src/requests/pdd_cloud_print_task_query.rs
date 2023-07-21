use crate::Request;

use serde::{Deserialize, Serialize};


/// 云打印任务查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CloudPrintTaskQuery {
    
    /// 打印序号，非必填，填了则只查询列表内的任务
    #[serde(rename = "print_sequence_list")]
    pub print_sequence_list: Option<Vec<i32>>,
    
    /// 打印任务id
    #[serde(rename = "print_task_id")]
    pub print_task_id: Option<String>,
    
    /// 打印机id
    #[serde(rename = "printer_id")]
    pub printer_id: Option<String>,
    
    /// 共享码
    #[serde(rename = "share_code")]
    pub share_code: Option<String>,
    
}

/// 云打印任务查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddCloudPrintTaskQuery {
    
    /// 云打印任务查询请求
    #[serde(rename = "cloud_print_task_query")]
    pub cloud_print_task_query: Option<CloudPrintTaskQuery>,
    
}


impl Request for PddCloudPrintTaskQuery {
    fn get_type() -> String {
        "pdd.cloud.print.task.query".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
