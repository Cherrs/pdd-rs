use crate::Request;

use serde::{Deserialize, Serialize};


/// 创建商品全站推广广告。默认使用智能创意投放注：同一店铺内所有全站推广相关的写接口不可并发调用
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitTrCreate {
    
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
    /// 日限额（厘），范围100000~1000000000（表示100-1000000元）,选填
    #[serde(rename = "maxCost")]
    pub max_cost: Option<i64>,
    
    /// 成交出价（厘），4000~1000000(表示4-1000元)，选填
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: Option<i64>,
    
    /// 目标roi 范围1000~1000000 (万分位，表示0.1~100)，选填
    #[serde(rename = "targetRoi")]
    pub target_roi: Option<i64>,
    
}


impl Request for PddAdApiUnitTrCreate {
    fn get_type() -> String {
        "pdd.ad.api.unit.tr.create".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
