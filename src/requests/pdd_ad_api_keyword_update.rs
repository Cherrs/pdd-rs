use crate::Request;

use serde::{Deserialize, Serialize};


/// 批量修改关键词
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Keywords {
    
    /// 关键词出价
    #[serde(rename = "bid")]
    pub bid: Option<i64>,
    
    /// 关键词Id
    #[serde(rename = "keywordId")]
    pub keyword_id: Option<i64>,
    
    /// 关键词溢价比例。万分比。
    #[serde(rename = "premiumRate")]
    pub premium_rate: Option<i64>,
    
}

/// 批量修改关键词
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiKeywordUpdate {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 关键词列表
    #[serde(rename = "keywords")]
    pub keywords: Option<Vec<Keywords>>,
    
}


impl Request for PddAdApiKeywordUpdate {
    fn get_type() -> String {
        "pdd.ad.api.keyword.update".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
