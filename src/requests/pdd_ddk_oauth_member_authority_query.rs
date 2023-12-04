use crate::Request;

use serde::{Deserialize, Serialize};


/// 通过pid和自定义参数来查询是否已经绑定备案
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkOauthMemberAuthorityQuery {
    
    /// 推广位id
    #[serde(rename = "pid")]
    pub pid: Option<String>,
    
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key
    #[serde(rename = "custom_parameters")]
    pub custom_parameters: Option<String>,
    
}


/// 通过pid和自定义参数来查询是否已经绑定备案
impl Request for PddDdkOauthMemberAuthorityQuery {
    fn get_type() -> String {
        "pdd.ddk.oauth.member.authority.query".to_string()
    }

    fn get_response_name() -> String {
        "authority_query_response".to_string()
    }
}
