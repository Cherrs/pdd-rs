use crate::Request;

use serde::{Deserialize, Serialize};


/// 快递公司查看接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsCompaniesGet {
    
}


impl Request for PddLogisticsCompaniesGet {
    fn get_type() -> String {
        "pdd.logistics.companies.get".to_string()
    }

    fn get_response_name() -> String {
        "logistics_companies_get_response".to_string()
    }
}
