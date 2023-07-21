use crate::Request;

use serde::{Deserialize, Serialize};


/// 更新全站推广名称注：同一店铺内所有全站推广相关的写接口不可并发调用
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitTrUpdateAdName {
    
    /// 广告名称
    #[serde(rename = "adName")]
    pub ad_name: Option<String>,
    
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddAdApiUnitTrUpdateAdName {
    fn get_type() -> String {
        "pdd.ad.api.unit.tr.update.ad.name".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
