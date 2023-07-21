use crate::Request;

use serde::{Deserialize, Serialize};


/// 商家打印跨境全托管商品标签码时查询标签码信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddSelectGoodsLabelCode {
    
    /// 履约单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
}


impl Request for PddSelectGoodsLabelCode {
    fn get_type() -> String {
        "pdd.select.goods.label.code".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
