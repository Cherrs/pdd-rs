use crate::Request;

use serde::{Deserialize, Serialize};


/// 检查入参号码是否订单所绑定的虚拟号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOpenVirtualNumberCheck {
    
    /// 分机号，非必填，4位数字字符
    #[serde(rename = "identify_number")]
    pub identify_number: Option<String>,
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 虚拟号，11位数字字符
    #[serde(rename = "virtual_number")]
    pub virtual_number: Option<String>,
    
}


/// 检查入参号码是否订单所绑定的虚拟号
impl Request for PddOpenVirtualNumberCheck {
    fn get_type() -> String {
        "pdd.open.virtual.number.check".to_string()
    }

    fn get_response_name() -> String {
        "open_virtual_number_check_response".to_string()
    }
}
