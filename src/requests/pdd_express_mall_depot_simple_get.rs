use crate::Request;

use serde::{Deserialize, Serialize};


/// 商家全部仓库的简要信息列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddExpressMallDepotSimpleGet {
    
}


/// 商家全部仓库的简要信息列表(无业务入参)
impl Request for PddExpressMallDepotSimpleGet {
    fn get_type() -> String {
        "pdd.express.mall.depot.simple.get".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
