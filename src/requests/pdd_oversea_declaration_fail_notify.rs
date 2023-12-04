use crate::Request;

use serde::{Deserialize, Serialize};


/// 用于服务商向平台同步海淘订单申报失败和具体原因
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOverseaDeclarationFailNotify {
    
    /// 1-超过购买额度，2-清关异常（如重量超标、退运、扣留等）
    #[serde(rename = "fail_reason")]
    pub fail_reason: Option<i32>,
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
}


/// 用于服务商向平台同步海淘订单申报失败和具体原因
impl Request for PddOverseaDeclarationFailNotify {
    fn get_type() -> String {
        "pdd.oversea.declaration.fail.notify".to_string()
    }

    fn get_response_name() -> String {
        "notify_exceeded_response".to_string()
    }
}
