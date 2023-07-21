use crate::Request;

use serde::{Deserialize, Serialize};


/// 开平短信服务发送短信接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOpenMsgServiceSendMsg {
    
    /// 业务请求唯一标识
    #[serde(rename = "out_id")]
    pub out_id: Option<String>,
    
    /// 接收短信的手机号码列表（仅允许密文）,["密文1", "密文2"]
    #[serde(rename = "phone_numbers")]
    pub phone_numbers: Option<Vec<String>>,
    
    /// 短信签名名称
    #[serde(rename = "sign_name")]
    pub sign_name: Option<String>,
    
    /// 上行短信扩展码
    #[serde(rename = "sms_up_extend_code")]
    pub sms_up_extend_code: Option<String>,
    
    /// 短信模板ID
    #[serde(rename = "template_code")]
    pub template_code: Option<i64>,
    
    /// 短信模板变量对应的实际值，JSON格式,示例："${param}","aaa"，注意${}符号勿遗漏
    #[serde(rename = "template_param")]
    pub template_param: Option<TemplateParam>,
    
}

/// 开平短信服务发送短信接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TemplateParam {
    
    /// 模板变量名
    #[serde(rename = "$key")]
    pub key: Option<String>,
    
    /// 模板变量值
    #[serde(rename = "$value")]
    pub value: Option<String>,
    
}


impl Request for PddOpenMsgServiceSendMsg {
    fn get_type() -> String {
        "pdd.open.msg.service.send.msg".to_string()
    }

    fn get_response_name() -> String {
        "code".to_string()
    }
}
