use crate::Request;

use serde::{Deserialize, Serialize};


/// 多多进宝推广短链解析
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkUrlShortParse {
    
    /// 需要解析出长链的多多进宝短连接，仅支持短链接（即为pdd.ddk.goods.promotion.url.generate接口生成的短链）
    #[serde(rename = "original_url")]
    pub original_url: Option<String>,
    
}


/// 多多进宝推广短链解析
impl Request for PddDdkUrlShortParse {
    fn get_type() -> String {
        "pdd.ddk.url.short.parse".to_string()
    }

    fn get_response_name() -> String {
        "url".to_string()
    }
}
