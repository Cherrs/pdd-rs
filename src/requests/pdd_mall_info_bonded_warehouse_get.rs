use crate::Request;

use serde::{Deserialize, Serialize};


/// 保税仓信息查询接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoBondedWarehouseGet {
    
}


impl Request for PddMallInfoBondedWarehouseGet {
    fn get_type() -> String {
        "pdd.mall.info.bonded.warehouse.get".to_string()
    }

    fn get_response_name() -> String {
        "mall_info_bonded_warehouse_get_response".to_string()
    }
}
