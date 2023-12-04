use crate::Request;

use serde::{Deserialize, Serialize};


/// 家电分仓库存-删除货品
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockWareDelete {
    
    /// 货品id
    #[serde(rename = "ware_id")]
    pub ware_id: Option<i64>,
    
}


/// 家电分仓库存-删除货品
impl Request for PddStockWareDelete {
    fn get_type() -> String {
        "pdd.stock.ware.delete".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
