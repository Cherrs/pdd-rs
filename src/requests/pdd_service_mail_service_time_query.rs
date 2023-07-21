use crate::Request;

use serde::{Deserialize, Serialize};


/// 服务时间查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddServiceMailServiceTimeQueryRequest {
    
    /// 省名称
    #[serde(rename = "provName")]
    pub prov_name: Option<String>,
    
    /// 市名称
    #[serde(rename = "cityName")]
    pub city_name: Option<String>,
    
    /// 区名称
    #[serde(rename = "districtName")]
    pub district_name: Option<String>,
    
    /// 街道名称
    #[serde(rename = "streetName")]
    pub street_name: Option<String>,
    
    /// 寄件类型
    #[serde(rename = "postType")]
    pub post_type: Option<String>,
    
    /// 收件省名称
    #[serde(rename = "receiveProvName")]
    pub receive_prov_name: Option<String>,
    
    /// 收件市名称
    #[serde(rename = "receiveCityName")]
    pub receive_city_name: Option<String>,
    
    /// 收件区名称
    #[serde(rename = "receiveDistrictName")]
    pub receive_district_name: Option<String>,
    
    /// 收件街道名称
    #[serde(rename = "receiveStreetName")]
    pub receive_street_name: Option<String>,
    
    /// 收件详细地址
    #[serde(rename = "receiveAddrDetail")]
    pub receive_addr_detail: Option<String>,
    
    /// 扩展信息； options如果不存在，说明不需要对发货地收货地校验
    #[serde(rename = "attributes")]
    pub attributes: Option<String>,
    
}

/// 服务时间查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddServiceMailServiceTimeQuery {
    
    /// 请求参数
    #[serde(rename = "request")]
    pub request: Option<PddServiceMailServiceTimeQueryRequest>,
    
}


impl Request for PddServiceMailServiceTimeQuery {
    fn get_type() -> String {
        "pdd.service.mail.service.time.query".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
