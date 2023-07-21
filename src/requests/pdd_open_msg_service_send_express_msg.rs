use crate::Request;

use serde::{Deserialize, Serialize};


/// 快递派送过程中根据物流编号发送短信通知
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOpenMsgServiceSendExpressMsg {
    
    /// 业务请求唯一标识
    #[serde(rename = "out_id")]
    pub out_id: Option<String>,
    
    /// 短信签名名称
    #[serde(rename = "sign_name")]
    pub sign_name: Option<String>,
    
    /// 上行短信扩展码
    #[serde(rename = "sms_up_extend_code")]
    pub sms_up_extend_code: Option<String>,
    
    /// 短信模板CODE
    #[serde(rename = "template_code")]
    pub template_code: Option<i64>,
    
    /// 短信模板变量JSON集合(与手机号对应)与按照手机号发短信一致key变量名 value变量值,示例："${param}","aaa"，注意${}符号勿遗漏
    #[serde(rename = "template_param_json")]
    pub template_param_json: Option<TemplateParamJson>,
    
    /// 物流单号集合
    #[serde(rename = "waybill_codes")]
    pub waybill_codes: Option<Vec<String>>,
    
    /// 快递公司编码
    #[serde(rename = "wp_code")]
    pub wp_code: Option<String>,
    
}

/// 快递派送过程中根据物流编号发送短信通知
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TemplateParamJson {
    
    /// 模板变量key
    #[serde(rename = "$key")]
    pub key: Option<String>,
    
    /// 模板变量value
    #[serde(rename = "$value")]
    pub value: Option<String>,
    
}


impl Request for PddOpenMsgServiceSendExpressMsg {
    fn get_type() -> String {
        "pdd.open.msg.service.send.express.msg".to_string()
    }

    fn get_response_name() -> String {
        "biz_id".to_string()
    }
}
