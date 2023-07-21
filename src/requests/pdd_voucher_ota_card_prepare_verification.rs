use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询平台生成卡密对应的卡券信息、商品信息和订单信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVoucherOtaCardPrepareVerificationRequest {
    
    /// 卡密
    #[serde(rename = "card_no")]
    pub card_no: Option<String>,
    
    /// 门店id
    #[serde(rename = "store_id")]
    pub store_id: Option<i64>,
    
}

/// 查询平台生成卡密对应的卡券信息、商品信息和订单信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVoucherOtaCardPrepareVerification {
    
    /// 请求体
    #[serde(rename = "request")]
    pub request: Option<PddVoucherOtaCardPrepareVerificationRequest>,
    
}


impl Request for PddVoucherOtaCardPrepareVerification {
    fn get_type() -> String {
        "pdd.voucher.ota.card.prepare.verification".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
