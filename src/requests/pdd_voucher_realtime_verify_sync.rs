use crate::Request;

use serde::{Deserialize, Serialize};


/// 用户使用券码时，商家需要实时给PDD侧回传券码核销结果
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVoucherRealtimeVerifySyncRequest {
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 备注
    #[serde(rename = "remark")]
    pub remark: Option<String>,
    
    /// ISV核销流水号
    #[serde(rename = "serial_no")]
    pub serial_no: Option<String>,
    
    /// 门店名称
    #[serde(rename = "shop_name")]
    pub shop_name: Option<String>,
    
    /// 门店编号
    #[serde(rename = "shop_no")]
    pub shop_no: Option<String>,
    
    /// 卡券核销时间（13 位毫秒）
    #[serde(rename = "verify_time")]
    pub verify_time: Option<i64>,
    
    /// 卡券编号
    #[serde(rename = "out_voucher_id")]
    pub out_voucher_id: Option<String>,
    
}

/// 用户使用券码时，商家需要实时给PDD侧回传券码核销结果
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVoucherRealtimeVerifySync {
    
    /// 请求入参
    #[serde(rename = "request")]
    pub request: Option<PddVoucherRealtimeVerifySyncRequest>,
    
}


impl Request for PddVoucherRealtimeVerifySync {
    fn get_type() -> String {
        "pdd.voucher.realtime.verify.sync".to_string()
    }

    fn get_response_name() -> String {
        "code".to_string()
    }
}
