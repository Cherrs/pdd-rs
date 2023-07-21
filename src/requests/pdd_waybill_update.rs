use crate::Request;

use serde::{Deserialize, Serialize};


/// 电子面单云打印更新接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PackageInfo {
    
    /// 商品
    #[serde(rename = "items")]
    pub items: Option<Vec<Items>>,
    
    /// 体积
    #[serde(rename = "volume")]
    pub volume: Option<i32>,
    
    /// 重量
    #[serde(rename = "weight")]
    pub weight: Option<i32>,
    
}

/// 电子面单云打印更新接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddWaybillUpdate {
    
    /// param_waybill_cloud_print_update_request
    #[serde(rename = "param_waybill_cloud_print_update_request")]
    pub param_waybill_cloud_print_update_request: Option<ParamWaybillCloudPrintUpdateRequest>,
    
}

/// 电子面单云打印更新接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Recipient {
    
    /// 地址
    #[serde(rename = "address")]
    pub address: Option<Address>,
    
    /// 手机号码
    #[serde(rename = "mobile")]
    pub mobile: Option<String>,
    
    /// 姓名
    #[serde(rename = "name")]
    pub name: Option<String>,
    
    /// 固定电话
    #[serde(rename = "phone")]
    pub phone: Option<String>,
    
}

/// 电子面单云打印更新接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Sender {
    
    /// 手机号码
    #[serde(rename = "mobile")]
    pub mobile: Option<String>,
    
    /// 姓名
    #[serde(rename = "name")]
    pub name: Option<String>,
    
    /// 固定电话
    #[serde(rename = "phone")]
    pub phone: Option<String>,
    
}

/// 电子面单云打印更新接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Address {
    
    /// 城市
    #[serde(rename = "city")]
    pub city: Option<String>,
    
    /// 地区/国家
    #[serde(rename = "country")]
    pub country: Option<String>,
    
    /// 详细地址
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    
    /// 区地址
    #[serde(rename = "district")]
    pub district: Option<String>,
    
    /// 省
    #[serde(rename = "province")]
    pub province: Option<String>,
    
    /// 街道
    #[serde(rename = "town")]
    pub town: Option<String>,
    
}

/// 电子面单云打印更新接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParamWaybillCloudPrintUpdateRequest {
    
    /// 请求表示id
    #[serde(rename = "object_id")]
    pub object_id: Option<String>,
    
    /// 包裹信息
    #[serde(rename = "package_info")]
    pub package_info: Option<PackageInfo>,
    
    /// 收件信息
    #[serde(rename = "recipient")]
    pub recipient: Option<Recipient>,
    
    /// 发件信息
    #[serde(rename = "sender")]
    pub sender: Option<Sender>,
    
    /// 模板URL
    #[serde(rename = "template_url")]
    pub template_url: Option<String>,
    
    /// 面单号
    #[serde(rename = "waybill_code")]
    pub waybill_code: Option<String>,
    
    /// 物流公司CODE
    #[serde(rename = "wp_code")]
    pub wp_code: Option<String>,
    
}

/// 电子面单云打印更新接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Items {
    
    /// 数量
    #[serde(rename = "count")]
    pub count: Option<i32>,
    
    /// 名称
    #[serde(rename = "name")]
    pub name: Option<String>,
    
}


impl Request for PddWaybillUpdate {
    fn get_type() -> String {
        "pdd.waybill.update".to_string()
    }

    fn get_response_name() -> String {
        "pdd_waybill_update_response".to_string()
    }
}
