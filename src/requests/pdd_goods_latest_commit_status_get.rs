use crate::Request;

use serde::{Deserialize, Serialize};


/// 批量goodsId查询最新的审核状态
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsLatestCommitStatusGet {
    
    /// 商品id(不超过100个)
    #[serde(rename = "goods_id_list")]
    pub goods_id_list: Option<Vec<i64>>,
    
}


/// 批量goodsId查询最新的审核状态
impl Request for PddGoodsLatestCommitStatusGet {
    fn get_type() -> String {
        "pdd.goods.latest.commit.status.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_latest_commit_status_get_response".to_string()
    }
}
