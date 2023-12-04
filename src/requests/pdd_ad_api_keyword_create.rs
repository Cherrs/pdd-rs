use crate::Request;

use serde::{Deserialize, Serialize};


/// 批量创建关键词
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KeywordList {
    
    /// 关键词出价
    #[serde(rename = "bid")]
    pub bid: Option<i64>,
    
    /// 关键词溢价比例。万分比。
    #[serde(rename = "premiumRate")]
    pub premium_rate: Option<i64>,
    
    /// 关键词
    #[serde(rename = "word")]
    pub word: Option<String>,
    
}

/// 批量创建关键词
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiKeywordCreate {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 关键词创建信息列表
    #[serde(rename = "keywordList")]
    pub keyword_list: Option<Vec<KeywordList>>,
    
}


/// 批量创建关键词
impl Request for PddAdApiKeywordCreate {
    fn get_type() -> String {
        "pdd.ad.api.keyword.create".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
