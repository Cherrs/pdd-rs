use crate::Request;

use serde::{Deserialize, Serialize};


/// 批量删除关键词
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiKeywordDelete {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 关键词Id列表
    #[serde(rename = "keywordIds")]
    pub keyword_ids: Option<Vec<i64>>,
    
}


impl Request for PddAdApiKeywordDelete {
    fn get_type() -> String {
        "pdd.ad.api.keyword.delete".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
