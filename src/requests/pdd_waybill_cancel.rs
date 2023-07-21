use crate::Request;

use serde::{Deserialize, Serialize};


/// 商家取消获取的电子面单号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddWaybillCancel {
    
    /// 电子面单号
    #[serde(rename = "waybill_code")]
    pub waybill_code: Option<String>,
    
    /// 快递公司code
    #[serde(rename = "wp_code")]
    pub wp_code: Option<String>,
    
}


impl Request for PddWaybillCancel {
    fn get_type() -> String {
        "pdd.waybill.cancel".to_string()
    }

    fn get_response_name() -> String {
        "pdd_waybill_cancel_response".to_string()
    }
}
