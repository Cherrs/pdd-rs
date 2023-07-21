use crate::Request;

use serde::{Deserialize, Serialize};


/// 增加仓库
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Value {
    
    /// 城市id
    #[serde(rename = "key")]
    pub key: Option<String>,
    
    /// 区id列表
    #[serde(rename = "value")]
    pub value: Option<Vec<String>>,
    
}

/// 增加仓库
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddExpressAddDepot {
    
    /// 联系人姓名
    #[serde(rename = "contact_name")]
    pub contact_name: Option<String>,
    
    /// 仓库详细地址5-20字
    #[serde(rename = "depot_address")]
    pub depot_address: Option<String>,
    
    /// 别名
    #[serde(rename = "depot_alias")]
    pub depot_alias: Option<String>,
    
    /// 所在市id
    #[serde(rename = "depot_city_id")]
    pub depot_city_id: Option<i32>,
    
    /// 仓库编码
    #[serde(rename = "depot_code")]
    pub depot_code: Option<String>,
    
    /// 所在区id
    #[serde(rename = "depot_district_id")]
    pub depot_district_id: Option<i32>,
    
    /// 仓库名称
    #[serde(rename = "depot_name")]
    pub depot_name: Option<String>,
    
    /// 所在省id
    #[serde(rename = "depot_province_id")]
    pub depot_province_id: Option<i32>,
    
    /// 仓库区域（ 省->市->区id列表）例如：{"34":{"396":[3383]}}Map<Integer, Map<Integer, List<Integer>>>{1:{  2:[3,4]  }}
    #[serde(rename = "depot_region")]
    pub depot_region: Option<DepotRegion>,
    
    /// 联系人电话
    #[serde(rename = "telephone")]
    pub telephone: Option<String>,
    
    /// 邮编
    #[serde(rename = "zip_code")]
    pub zip_code: Option<String>,
    
}

/// 增加仓库
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DepotRegion {
    
    /// 省份id
    #[serde(rename = "key")]
    pub key: Option<String>,
    
    /// 市 -> 区id列表
    #[serde(rename = "value")]
    pub value: Option<Value>,
    
}


impl Request for PddExpressAddDepot {
    fn get_type() -> String {
        "pdd.express.add.depot".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
