use crate::Request;

use serde::{Deserialize, Serialize};


/// 门票商品查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTicketGoodsQuery {
    
    /// 草稿id，入参草稿id时，表示查询该草稿的信息
    #[serde(rename = "goods_commit_id")]
    pub goods_commit_id: Option<i64>,
    
    /// 商品id入参商品id时，表示查询该商品的线上商品信息。。
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


/// 门票商品查询
impl Request for PddTicketGoodsQuery {
    fn get_type() -> String {
        "pdd.ticket.goods.query".to_string()
    }

    fn get_response_name() -> String {
        "goods_detail_get_response".to_string()
    }
}
