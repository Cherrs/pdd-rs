use crate::Request;

use serde::{Deserialize, Serialize};


/// 更新全站推广日预算注：同一店铺内所有全站推广相关的写接口不可并发调用
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitTrUpdateMaxCost {
    
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
    /// 单日消耗上线（厘），范围100000~1000000000（表示100-1000000元）
    #[serde(rename = "maxCost")]
    pub max_cost: Option<i64>,
    
}


impl Request for PddAdApiUnitTrUpdateMaxCost {
    fn get_type() -> String {
        "pdd.ad.api.unit.tr.update.max.cost".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
