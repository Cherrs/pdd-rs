use crate::Request;

use serde::{Deserialize, Serialize};


/// ISV多店铺关联时，被关联店铺同意关联后产生关联code，ISV应用使用此code换取被关联店铺Access Token
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPopMallBindTokenGet {
    
    /// 被关联店铺的关联code
    #[serde(rename = "bind_code")]
    pub bind_code: Option<String>,
    
    /// 三方应用的用户id
    #[serde(rename = "external_uid")]
    pub external_uid: Option<String>,
    
    /// 当前店群包含的拼多多店铺id
    #[serde(rename = "mall_list")]
    pub mall_list: Option<Vec<i64>>,
    
}


/// ISV多店铺关联时，被关联店铺同意关联后产生关联code，ISV应用使用此code换取被关联店铺Access Token
impl Request for PddPopMallBindTokenGet {
    fn get_type() -> String {
        "pdd.pop.mall.bind.token.get".to_string()
    }

    fn get_response_name() -> String {
        "pop_auth_token_create_response".to_string()
    }
}
