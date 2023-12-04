use crate::Request;

use serde::{Deserialize, Serialize};


/// 获得拼多多商品标签列表（非商品类目cat，当前仅开放给多多客使用）
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsOptGet {
    
    /// 值=0时为顶点opt_id,通过树顶级节点获取opt树
    #[serde(rename = "parent_opt_id")]
    pub parent_opt_id: Option<i32>,
    
}


/// 获得拼多多商品标签列表（非商品类目cat，当前仅开放给多多客使用）
impl Request for PddGoodsOptGet {
    fn get_type() -> String {
        "pdd.goods.opt.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_opt_get_response".to_string()
    }
}
