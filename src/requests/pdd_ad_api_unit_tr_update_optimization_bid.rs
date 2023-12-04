
use crate::Request;
use serde::{Deserialize, Serialize};

/// 更新全站推广成交出价注：同一店铺内所有全站推广相关的写接口不可并发调用
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitTrUpdateOptimizationBid {
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,

    /// 成交出价，4000~1000000(表示4-1000元)
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: Option<i64>,
}

/// 更新全站推广成交出价
/// 注：同一店铺内所有全站推广相关的写接口不可并发调用
impl Request for PddAdApiUnitTrUpdateOptimizationBid {
    fn get_type() -> String {
        "pdd.ad.api.unit.tr.update.optimization.bid".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
