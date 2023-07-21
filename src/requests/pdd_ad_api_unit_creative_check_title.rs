use crate::Request;

use serde::{Deserialize, Serialize};


/// 检查创意标题是否合法
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitCreativeCheckTitle {
    
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
    /// 创意标题
    #[serde(rename = "title")]
    pub title: Option<String>,
    
}


impl Request for PddAdApiUnitCreativeCheckTitle {
    fn get_type() -> String {
        "pdd.ad.api.unit.creative.check.title".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
