use crate::Request;

use serde::{Deserialize, Serialize};


/// 旅游门票区域编码查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTicketAreacodeGet {
    
}


/// 供应商获取pdd的区域编码
impl Request for PddTicketAreacodeGet {
    fn get_type() -> String {
        "pdd.ticket.areacode.get".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
