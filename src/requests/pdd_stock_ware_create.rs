use crate::Request;

use serde::{Deserialize, Serialize};


/// 家电分仓库存-创建货品
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockWareCreate {
    
    /// 类型 0:单独货品。1:组合货品
    #[serde(rename = "ware_type")]
    pub ware_type: Option<i32>,
    
    /// 组合货品中子货品的关联关系, ware_type为1时必填；
    #[serde(rename = "ware_infos")]
    pub ware_infos: Option<Vec<WareInfos>>,
    
    /// 货品编码
    #[serde(rename = "ware_sn")]
    pub ware_sn: Option<String>,
    
    /// 货品名称
    #[serde(rename = "ware_name")]
    pub ware_name: Option<String>,
    
    /// 备注
    #[serde(rename = "note")]
    pub note: Option<String>,
    
    /// 高低值服务
    #[serde(rename = "service_quality")]
    pub service_quality: Option<i32>,
    
    /// 体积：立方毫米，只精确到100（即：最末两位为0）
    #[serde(rename = "volume")]
    pub volume: Option<i32>,
    
    /// 长：毫米，精确到1
    #[serde(rename = "length")]
    pub length: Option<i32>,
    
    /// 宽：毫米，精确到1
    #[serde(rename = "width")]
    pub width: Option<i32>,
    
    /// 高：毫米，精确到1
    #[serde(rename = "height")]
    pub height: Option<i32>,
    
    /// 重量：g，精确到10（即：末位为0）
    #[serde(rename = "weight")]
    pub weight: Option<i32>,
    
    /// 毛重：g，精确到10（即：末位为0）
    #[serde(rename = "gross_weight")]
    pub gross_weight: Option<i32>,
    
    /// 净重：g，精确到10（即：末位为0）
    #[serde(rename = "net_weight")]
    pub net_weight: Option<i32>,
    
    /// 皮重：g，精确到10（即：末位为0）
    #[serde(rename = "tare_weight")]
    pub tare_weight: Option<i32>,
    
    /// 单价：分，精确到10（即：末位为0）
    #[serde(rename = "price")]
    pub price: Option<i32>,
    
    /// 颜色
    #[serde(rename = "color")]
    pub color: Option<String>,
    
    /// 包材
    #[serde(rename = "packing")]
    pub packing: Option<String>,
    
}

/// 家电分仓库存-创建货品
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WareInfos {
    
    /// 
    #[serde(rename = "ware_quantity")]
    pub ware_quantity: Option<i32>,
    
    /// 
    #[serde(rename = "ware_id")]
    pub ware_id: Option<i64>,
    
}


/// 家电分仓库存-创建货品
impl Request for PddStockWareCreate {
    fn get_type() -> String {
        "pdd.stock.ware.create".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
