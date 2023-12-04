use crate::Request;

use serde::{Deserialize, Serialize};


/// 给商家提供可发货的快递公司，此数据仅作参考，如返回为空不代表不可发货
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsAvailableCompanyRecommend {
    
    /// 收件人市id（最多支持50个）
    #[serde(rename = "city_id")]
    pub city_id: Option<Vec<i64>>,
    
}


/// 给商家提供可发货的快递公司，此数据仅作参考，如返回为空不代表不可发货
impl Request for PddLogisticsAvailableCompanyRecommend {
    fn get_type() -> String {
        "pdd.logistics.available.company.recommend".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
