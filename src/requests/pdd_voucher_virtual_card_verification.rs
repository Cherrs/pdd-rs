use crate::Request;

use serde::{Deserialize, Serialize};


/// 卡券（电子）核销接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VoucherDataList {
    
    /// 流水号
    #[serde(rename = "out_trans_no")]
    pub out_trans_no: Option<String>,
    
    /// 券状态更改时间
    #[serde(rename = "voucher_time")]
    pub voucher_time: Option<i64>,
    
    /// 券状态 1：已核销；2：已销毁
    #[serde(rename = "voucher_status")]
    pub voucher_status: Option<i32>,
    
    /// 券号
    #[serde(rename = "voucher_no")]
    pub voucher_no: Option<String>,
    
}

/// 卡券（电子）核销接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVoucherVirtualCardVerification {
    
    /// 拼多多订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 券信息列表
    #[serde(rename = "voucher_data_list")]
    pub voucher_data_list: Option<Vec<VoucherDataList>>,
    
}


impl Request for PddVoucherVirtualCardVerification {
    fn get_type() -> String {
        "pdd.voucher.virtual.card.verification".to_string()
    }

    fn get_response_name() -> String {
        "voucher_voucher_info_verify_response".to_string()
    }
}
