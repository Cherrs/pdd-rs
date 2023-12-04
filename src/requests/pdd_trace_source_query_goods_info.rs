use crate::Request;

use serde::{Deserialize, Serialize};


/// 根据溯源码ID获取溯源商品信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Params {
    
    /// 接口调用账号（由平台分配）
    #[serde(rename = "userid")]
    pub userid: Option<String>,
    
    /// 请求时间戳，10分钟有效，格式：yyyy-MM-dd HH:mm:ss
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
    
    /// 签名
    #[serde(rename = "sign")]
    pub sign: Option<String>,
    
    /// 防伪溯源码ID
    #[serde(rename = "id")]
    pub id: Option<String>,
    
}

/// 根据溯源码ID获取溯源商品信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTraceSourceQueryGoodsInfo {
    
    /// 请求方法
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    
    /// 请求参数
    #[serde(rename = "params")]
    pub params: Option<Params>,
    
}


/// 根据溯源码ID获取溯源商品信息
impl Request for PddTraceSourceQueryGoodsInfo {
    fn get_type() -> String {
        "pdd.trace.source.query.goods.info".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
