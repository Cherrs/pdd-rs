use crate::Request;

use serde::{Deserialize, Serialize};


/// 重抛直邮订单，如果产生逆向物流轨迹，供应商回传逆向运单信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddHeavygoodsBackExpressCallback {
    
    /// 包裹回退原因
    #[serde(rename = "back_reason")]
    pub back_reason: Option<String>,
    
    /// 逆向运单号
    #[serde(rename = "back_trck_no")]
    pub back_trck_no: Option<String>,
    
    /// 逆向运单所关联的正向运单号
    #[serde(rename = "trck_no")]
    pub trck_no: Option<String>,
    
}


/// 重抛直邮订单，如果产生逆向物流轨迹，供应商回传逆向运单信息
impl Request for PddHeavygoodsBackExpressCallback {
    fn get_type() -> String {
        "pdd.heavygoods.back.express.callback".to_string()
    }

    fn get_response_name() -> String {
        "error_code".to_string()
    }
}
