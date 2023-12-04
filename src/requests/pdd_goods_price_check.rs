use crate::Request;

use serde::{Deserialize, Serialize};


/// 提供商品价格核实功能
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsPriceCheck {
    
    /// 商品id，long值，大于0
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
}


/// 提供商品价格核实功能
impl Request for PddGoodsPriceCheck {
    fn get_type() -> String {
        "pdd.goods.price.check".to_string()
    }

    fn get_response_name() -> String {
        "goodsid_price_check_response".to_string()
    }
}
