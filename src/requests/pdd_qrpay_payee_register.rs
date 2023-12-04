use crate::Request;

use serde::{Deserialize, Serialize};


/// 交易二维码订单同店铺下需要记录订单来源的业务场景，可以将参数定义为门店、柜员、店员等
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddQrpayPayeeRegister {
    
    /// 参数列表
    #[serde(rename = "payee_list")]
    pub payee_list: Option<Vec<PayeeList>>,
    
}

/// 交易二维码订单同店铺下需要记录订单来源的业务场景，可以将参数定义为门店、柜员、店员等
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PayeeList {
    
    /// 参数名，用于注册到名单，并生成对应URL
    #[serde(rename = "payee")]
    pub payee: Option<String>,
    
}


/// 交易二维码订单同店铺下需要记录订单来源的业务场景，可以将参数定义为门店、柜员、店员等
impl Request for PddQrpayPayeeRegister {
    fn get_type() -> String {
        "pdd.qrpay.payee.register".to_string()
    }

    fn get_response_name() -> String {
        "qrpay_payee_register_response".to_string()
    }
}
