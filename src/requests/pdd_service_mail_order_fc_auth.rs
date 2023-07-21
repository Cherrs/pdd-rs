use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取丰巢开平的access_token
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddServiceMailOrderFcAuth {
    
    /// 拼接到url的参数
    #[serde(rename = "urlParams")]
    pub url_params: Option<UrlParams>,
    
    /// 请求方法
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    
}

/// 获取丰巢开平的access_token
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UrlParams {
    
    /// 丰巢开平app_key
    #[serde(rename = "app_key")]
    pub app_key: Option<String>,
    
    /// 丰巢开平app_secret
    #[serde(rename = "app_secret")]
    pub app_secret: Option<String>,
    
}


impl Request for PddServiceMailOrderFcAuth {
    fn get_type() -> String {
        "pdd.service.mail.order.fc.auth".to_string()
    }

    fn get_response_name() -> String {
        "access_token".to_string()
    }
}
