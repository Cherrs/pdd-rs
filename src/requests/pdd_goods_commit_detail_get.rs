use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品编辑或者提交之后，可以通过此接口查询提交后的编辑信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCommitDetailGet {
    
    /// 提交申请的序列id
    #[serde(rename = "goods_commit_id")]
    pub goods_commit_id: Option<i64>,
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddGoodsCommitDetailGet {
    fn get_type() -> String {
        "pdd.goods.commit.detail.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_commit_detail_response".to_string()
    }
}
