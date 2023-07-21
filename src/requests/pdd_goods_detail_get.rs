use crate::Request;

use serde::{Deserialize, Serialize};


/// 新增商品或者修改商品的时候
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsDetailGet {
    
    /// 1213414
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddGoodsDetailGet {
    fn get_type() -> String {
        "pdd.goods.detail.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_detail_get_response".to_string()
    }
}
