use crate::Request;

use serde::{Deserialize, Serialize};


/// 删除草稿接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDeleteDraftCommit {
    
    /// 草稿id
    #[serde(rename = "goods_commit_id")]
    pub goods_commit_id: Option<i64>,
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


/// 删除草稿接口
impl Request for PddDeleteDraftCommit {
    fn get_type() -> String {
        "pdd.delete.draft.commit".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
