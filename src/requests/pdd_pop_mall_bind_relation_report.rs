use crate::Request;

use serde::{Deserialize, Serialize};


/// ISV上报存量店铺关联关系
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPopMallBindRelationReport {
    
    /// 关联时间
    #[serde(rename = "bind_at")]
    pub bind_at: Option<i64>,
    
    /// 关联类型：0-关联码关联，1-授权关联
    #[serde(rename = "bind_type")]
    pub bind_type: Option<i32>,
    
    /// 三方应用的用户id
    #[serde(rename = "external_uid")]
    pub external_uid: Option<String>,
    
    /// 被关联方的店铺id
    #[serde(rename = "invitee_mall_id")]
    pub invitee_mall_id: Option<i64>,
    
    /// 发起关联方的店铺id
    #[serde(rename = "inviter_mall_id")]
    pub inviter_mall_id: Option<i64>,
    
    /// 当前店群包含的拼多多店铺id
    #[serde(rename = "mall_list")]
    pub mall_list: Option<Vec<i64>>,
    
}


impl Request for PddPopMallBindRelationReport {
    fn get_type() -> String {
        "pdd.pop.mall.bind.relation.report".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
