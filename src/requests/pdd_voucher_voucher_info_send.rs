use crate::Request;

use serde::{Deserialize, Serialize};


/// 第三方ISV将消费者购买的卡券信息同步给平台
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVoucherVoucherInfoSend {
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 外部流水号
    #[serde(rename = "out_biz_no")]
    pub out_biz_no: Option<String>,
    
    /// 卡券信息列表,例子[{"voucher_id":"test voucher_id","voucher_no":"test voucher_no"}]
    #[serde(rename = "voucher_list")]
    pub voucher_list: Option<Vec<VoucherList>>,
    
}

/// 第三方ISV将消费者购买的卡券信息同步给平台
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VoucherList {
    
    /// 卡券ID
    #[serde(rename = "voucher_id")]
    pub voucher_id: Option<String>,
    
    /// 卡券号
    #[serde(rename = "voucher_no")]
    pub voucher_no: Option<String>,
    
}


impl Request for PddVoucherVoucherInfoSend {
    fn get_type() -> String {
        "pdd.voucher.voucher.info.send".to_string()
    }

    fn get_response_name() -> String {
        "voucher_voucher_info_send_response".to_string()
    }
}
