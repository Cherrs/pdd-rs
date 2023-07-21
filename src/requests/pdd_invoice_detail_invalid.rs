use crate::Request;

use serde::{Deserialize, Serialize};


/// 根据订单号冲红发票
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddInvoiceDetailInvalid {
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
}


impl Request for PddInvoiceDetailInvalid {
    fn get_type() -> String {
        "pdd.invoice.detail.invalid".to_string()
    }

    fn get_response_name() -> String {
        "invoice_detail_invalid_response".to_string()
    }
}
