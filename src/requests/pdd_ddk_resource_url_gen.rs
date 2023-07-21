use crate::Request;

use serde::{Deserialize, Serialize};


/// 生成拼多多主站频道推广
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkResourceUrlGen {
    
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key
    #[serde(rename = "custom_parameters")]
    pub custom_parameters: Option<String>,
    
    /// 是否返回 schema URL
    #[serde(rename = "generate_schema_url")]
    pub generate_schema_url: Option<bool>,
    
    /// 是否生成拼多多福利券微信小程序推广信息
    #[serde(rename = "generate_we_app")]
    pub generate_we_app: Option<bool>,
    
    /// 推广位
    #[serde(rename = "pid")]
    pub pid: Option<String>,
    
    /// 频道来源：4-限时秒杀,39997-充值中心, 39998-活动转链，39996-百亿补贴，39999-电器城，40000-领券中心，50005-火车票
    #[serde(rename = "resource_type")]
    pub resource_type: Option<i32>,
    
    /// 原链接
    #[serde(rename = "url")]
    pub url: Option<String>,
    
}


impl Request for PddDdkResourceUrlGen {
    fn get_type() -> String {
        "pdd.ddk.resource.url.gen".to_string()
    }

    fn get_response_name() -> String {
        "resource_url_response".to_string()
    }
}
