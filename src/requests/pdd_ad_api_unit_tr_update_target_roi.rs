
use crate::Request;
use serde::{Deserialize, Serialize};

/// 更新全站推广targetRoi注：同一店铺内所有全站推广相关的写接口不可并发调用
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitTrUpdateTargetRoi {
    /// 商品id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,

    /// 目标roi 范围1000~1000000(万分位，表示0.1~100)
    #[serde(rename = "targetRoi")]
    pub target_roi: Option<i64>,
}

/// 更新全站推广targetRoi
/// 注：同一店铺内所有全站推广相关的写接口不可并发调用
impl Request for PddAdApiUnitTrUpdateTargetRoi {
    fn get_type() -> String {
        "pdd.ad.api.unit.tr.update.target.roi".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
