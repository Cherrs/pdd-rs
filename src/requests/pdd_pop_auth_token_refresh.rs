use crate::Request;

use serde::{Deserialize, Serialize};


/// 根据refresh_token重新生成token
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPopAuthTokenRefresh {
    
    /// grantType==refresh_token 时需要
    #[serde(rename = "refresh_token")]
    pub refresh_token: Option<String>,
    
}


impl Request for PddPopAuthTokenRefresh {
    fn get_type() -> String {
        "pdd.pop.auth.token.refresh".to_string()
    }

    fn get_response_name() -> String {
        "pop_auth_token_refresh_response".to_string()
    }
}
