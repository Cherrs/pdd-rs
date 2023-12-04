use crate::Request;

use serde::{Deserialize, Serialize};


/// 通过面单号查询面单信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParamList {
    
    /// 请求id
    #[serde(rename = "object_id")]
    pub object_id: Option<String>,
    
    /// 电子面单号
    #[serde(rename = "waybill_code")]
    pub waybill_code: Option<String>,
    
    /// 快递公司code
    #[serde(rename = "wp_code")]
    pub wp_code: Option<String>,
    
}

/// 通过面单号查询面单信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddWaybillQueryByWaybillcode {
    
    /// 系统自动生成
    #[serde(rename = "param_list")]
    pub param_list: Option<Vec<ParamList>>,
    
}


/// 通过面单号查询面单信息
impl Request for PddWaybillQueryByWaybillcode {
    fn get_type() -> String {
        "pdd.waybill.query.by.waybillcode".to_string()
    }

    fn get_response_name() -> String {
        "pdd_waybill_query_by_waybillcode_response".to_string()
    }
}
