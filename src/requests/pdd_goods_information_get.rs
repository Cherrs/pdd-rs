use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品详情（此接口后续不再维护，请使用pdd.goods.detail.get接口）
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsInformationGet {
    
    /// 商品编码
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddGoodsInformationGet {
    fn get_type() -> String {
        "pdd.goods.information.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_info_get_response".to_string()
    }
}
