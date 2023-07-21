use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取货品列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockWareInfoList {
    
    /// 货品编码集合
    #[serde(rename = "ware_sn_list")]
    pub ware_sn_list: Option<Vec<String>>,
    
}


impl Request for PddStockWareInfoList {
    fn get_type() -> String {
        "pdd.stock.ware.info.list".to_string()
    }

    fn get_response_name() -> String {
        "stock_ware_info_list_response".to_string()
    }
}
