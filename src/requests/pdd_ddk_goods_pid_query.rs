use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询已经生成的推广位信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkGoodsPidQuery {
    
    /// 返回的页数
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 返回的每页推广位数量
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 推广位列表，例如：["60005_612"]
    #[serde(rename = "pid_list")]
    pub pid_list: Option<Vec<String>>,
    
    /// 推广位状态：0-正常，1-封禁
    #[serde(rename = "status")]
    pub status: Option<i32>,
    
}


/// 查询已经生成的推广位信息
impl Request for PddDdkGoodsPidQuery {
    fn get_type() -> String {
        "pdd.ddk.goods.pid.query".to_string()
    }

    fn get_response_name() -> String {
        "p_id_query_response".to_string()
    }
}
