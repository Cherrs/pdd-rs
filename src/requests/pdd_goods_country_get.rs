use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品地区/国家接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCountryGet {
    
}


impl Request for PddGoodsCountryGet {
    fn get_type() -> String {
        "pdd.goods.country.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_country_get_response".to_string()
    }
}
