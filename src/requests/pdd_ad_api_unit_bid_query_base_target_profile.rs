use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取可用基础定向
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitBidQueryBaseTargetProfile {
    
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
    /// 场景类型，0-搜索，2-展示
    #[serde(rename = "scenesType")]
    pub scenes_type: Option<i32>,
    
}


/// 获取可用基础定向
impl Request for PddAdApiUnitBidQueryBaseTargetProfile {
    fn get_type() -> String {
        "pdd.ad.api.unit.bid.query.base.target.profile".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
