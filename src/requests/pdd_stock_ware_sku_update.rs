use crate::Request;

use serde::{Deserialize, Serialize};


/// 家电分仓库存-货品关联sku
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockWareSkuUpdate {
    
    /// 货品id
    #[serde(rename = "ware_id")]
    pub ware_id: Option<i64>,
    
    /// 组合货品中子货品的关联关系
    #[serde(rename = "ware_skus")]
    pub ware_skus: Option<Vec<WareSkus>>,
    
}

/// 家电分仓库存-货品关联sku
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WareSkus {
    
    /// sku id
    #[serde(rename = "sku_id")]
    pub sku_id: Option<i64>,
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddStockWareSkuUpdate {
    fn get_type() -> String {
        "pdd.stock.ware.sku.update".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
