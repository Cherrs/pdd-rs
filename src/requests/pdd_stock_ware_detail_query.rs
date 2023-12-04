use crate::Request;

use serde::{Deserialize, Serialize};


/// 家电分仓库存-查看货品详情
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockWareDetailQuery {
    
    /// 货品id
    #[serde(rename = "ware_id")]
    pub ware_id: Option<i64>,
    
}


/// 家电分仓库存-查看货品详情
impl Request for PddStockWareDetailQuery {
    fn get_type() -> String {
        "pdd.stock.ware.detail.query".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
