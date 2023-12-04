use crate::Request;

use serde::{Deserialize, Serialize};


/// 关键词相关性查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiKeywordRelevanceGet {
    
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
    /// 关键词数组
    #[serde(rename = "words")]
    pub words: Option<Vec<String>>,
    
}


/// 关键词相关性查询
impl Request for PddAdApiKeywordRelevanceGet {
    fn get_type() -> String {
        "pdd.ad.api.keyword.relevance.get".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
