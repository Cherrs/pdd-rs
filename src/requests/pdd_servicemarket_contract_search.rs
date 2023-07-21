use crate::Request;

use serde::{Deserialize, Serialize};


/// 用于查询指定商家在服务市场订单执行履约的排序
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddServicemarketContractSearch {
    
    /// 买家店铺id
    #[serde(rename = "mallId")]
    pub mall_id: Option<i64>,
    
}


impl Request for PddServicemarketContractSearch {
    fn get_type() -> String {
        "pdd.servicemarket.contract.search".to_string()
    }

    fn get_response_name() -> String {
        "servicemarket_contract_search_response".to_string()
    }
}
