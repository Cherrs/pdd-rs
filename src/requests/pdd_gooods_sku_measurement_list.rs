use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品sku计量单位枚举
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGooodsSkuMeasurementList {
    
}


impl Request for PddGooodsSkuMeasurementList {
    fn get_type() -> String {
        "pdd.gooods.sku.measurement.list".to_string()
    }

    fn get_response_name() -> String {
        "gooods_sku_measurement_list_response".to_string()
    }
}
