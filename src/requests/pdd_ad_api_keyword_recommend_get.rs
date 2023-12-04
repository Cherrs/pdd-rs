use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取推荐关键词
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiKeywordRecommendGet {
    
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
}


/// 获取推荐关键词
impl Request for PddAdApiKeywordRecommendGet {
    fn get_type() -> String {
        "pdd.ad.api.keyword.recommend.get".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
