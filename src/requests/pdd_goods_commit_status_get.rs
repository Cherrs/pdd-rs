use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询店铺的商品草稿列表状态
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCommitStatusGet {
    
    /// goods_commit_id列表
    #[serde(rename = "goods_commit_id_list")]
    pub goods_commit_id_list: Option<Vec<i64>>,
    
}


/// 查询店铺的商品草稿列表状态
impl Request for PddGoodsCommitStatusGet {
    fn get_type() -> String {
        "pdd.goods.commit.status.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_commit_status_get_response".to_string()
    }
}
