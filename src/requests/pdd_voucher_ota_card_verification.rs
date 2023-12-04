use crate::Request;

use serde::{Deserialize, Serialize};


/// 平台生成卡密类卡券核销
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVoucherOtaCardVerification {
    
    /// 待核销的券码
    #[serde(rename = "card_no")]
    pub card_no: Option<String>,
    
    /// 核销门店id
    #[serde(rename = "store_id")]
    pub store_id: Option<i64>,
    
    /// 核销门店名称
    #[serde(rename = "store_name")]
    pub store_name: Option<String>,
    
    /// 拼多多订单编号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
}


/// 平台生成卡密类卡券核销
impl Request for PddVoucherOtaCardVerification {
    fn get_type() -> String {
        "pdd.voucher.ota.card.verification".to_string()
    }

    fn get_response_name() -> String {
        "voucher_ota_card_verification_response".to_string()
    }
}
