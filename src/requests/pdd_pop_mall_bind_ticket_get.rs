use crate::Request;

use serde::{Deserialize, Serialize};


/// ISV多店铺关联时，获取发起方店铺身份ticket，用于生成店铺关联链接
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPopMallBindTicketGet {
    
    /// 三方应用的用户id
    #[serde(rename = "external_uid")]
    pub external_uid: Option<String>,
    
    /// 当前店群包含的拼多多店铺id
    #[serde(rename = "mall_list")]
    pub mall_list: Option<Vec<i64>>,
    
}


/// ISV多店铺关联时，获取发起方店铺身份ticket，用于生成店铺关联链接
impl Request for PddPopMallBindTicketGet {
    fn get_type() -> String {
        "pdd.pop.mall.bind.ticket.get".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
