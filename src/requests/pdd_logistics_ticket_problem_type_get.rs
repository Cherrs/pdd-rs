use crate::Request;

use serde::{Deserialize, Serialize};


/// 快递公司工单问题类型列表接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsTicketProblemTypeGet {
    
}


/// 快递公司通过此接口同步多多所有物流工单问题类型
impl Request for PddLogisticsTicketProblemTypeGet {
    fn get_type() -> String {
        "pdd.logistics.ticket.problem.type.get".to_string()
    }

    fn get_response_name() -> String {
        "logistics_problem_type_get_response".to_string()
    }
}
