use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询店铺的商品草稿列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCommitListGet {
    
    /// 草稿状态（0:编辑中,1:审核中,2:审核通过,3:审核驳回）
    #[serde(rename = "check_status")]
    pub check_status: Option<i32>,
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 页码，最多不超过100
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 每页数量，最多不超过100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
}


/// 查询店铺的商品草稿列表
impl Request for PddGoodsCommitListGet {
    fn get_type() -> String {
        "pdd.goods.commit.list.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_commit_list_get_response".to_string()
    }
}
