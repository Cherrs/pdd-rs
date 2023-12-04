
use crate::Request;
use serde::{Deserialize, Serialize};

/// 删除全站推广单元注：同一店铺内所有全站推广相关的写接口不可并发调用
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitTrDelete {
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
}

/// 删除全站推广单元
/// 注：同一店铺内所有全站推广相关的写接口不可并发调用
impl Request for PddAdApiUnitTrDelete {
    fn get_type() -> String {
        "pdd.ad.api.unit.tr.delete".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
