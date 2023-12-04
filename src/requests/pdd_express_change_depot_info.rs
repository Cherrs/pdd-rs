use crate::Request;

use serde::{Deserialize, Serialize};


/// 修改仓库信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddExpressChangeDepotInfo {
    
    /// 仓库id
    #[serde(rename = "depot_id")]
    pub depot_id: Option<i64>,
    
    /// 仓库编码
    #[serde(rename = "depot_code")]
    pub depot_code: Option<String>,
    
    /// 仓库名称
    #[serde(rename = "depot_name")]
    pub depot_name: Option<String>,
    
    /// 别名
    #[serde(rename = "depot_alias")]
    pub depot_alias: Option<String>,
    
    /// 所在省id
    #[serde(rename = "depot_province_id")]
    pub depot_province_id: Option<i32>,
    
    /// 所在市id
    #[serde(rename = "depot_city_id")]
    pub depot_city_id: Option<i32>,
    
    /// 所在区id
    #[serde(rename = "depot_district_id")]
    pub depot_district_id: Option<i32>,
    
    /// 仓库详细地址 5-20字
    #[serde(rename = "depot_address")]
    pub depot_address: Option<String>,
    
    /// 联系人姓名
    #[serde(rename = "contact_name")]
    pub contact_name: Option<String>,
    
    /// 联系人电话
    #[serde(rename = "telephone")]
    pub telephone: Option<String>,
    
    /// 仓库区域（ 省->市->区id列表）例如：{"34":{"396":[3383]}}Map<Integer, Map<Integer, List<Integer>>>
    #[serde(rename = "depot_region")]
    pub depot_region: Option<String>,
    
}


/// 修改仓库信息
impl Request for PddExpressChangeDepotInfo {
    fn get_type() -> String {
        "pdd.express.change.depot.info".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
