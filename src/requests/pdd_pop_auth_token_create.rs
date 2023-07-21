use crate::Request;

use serde::{Deserialize, Serialize};


/// 用户通过code换获取access_token
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPopAuthTokenCreate {
    
    /// 授权code，grantType==authorization_code 时需要
    #[serde(rename = "code")]
    pub code: Option<String>,
    
}


impl Request for PddPopAuthTokenCreate {
    fn get_type() -> String {
        "pdd.pop.auth.token.create".to_string()
    }

    fn get_response_name() -> String {
        "pop_auth_token_create_response".to_string()
    }
}
