use crate::Request;

use serde::{Deserialize, Serialize};


/// 修改订单收件地址 注：风险订单或订单已发货后不可修改
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOrderUpdateAddress {
    
    /// 收件详细地址
    #[serde(rename = "address")]
    pub address: Option<String>,
    
    /// 收件地城市
    #[serde(rename = "city")]
    pub city: Option<String>,
    
    /// 城市编码
    #[serde(rename = "city_id")]
    pub city_id: Option<i32>,
    
    /// 订单编号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 收件地省份
    #[serde(rename = "province")]
    pub province: Option<String>,
    
    /// 省份编码
    #[serde(rename = "province_id")]
    pub province_id: Option<i32>,
    
    /// 收件人姓名
    #[serde(rename = "receiver_name")]
    pub receiver_name: Option<String>,
    
    /// 收件人电话，明文
    #[serde(rename = "receiver_phone")]
    pub receiver_phone: Option<String>,
    
    /// 收件地区县
    #[serde(rename = "town")]
    pub town: Option<String>,
    
    /// 区县编码
    #[serde(rename = "town_id")]
    pub town_id: Option<i32>,
    
}


/// 修改订单收件地址 注：风险订单或订单已发货后不可修改
impl Request for PddOrderUpdateAddress {
    fn get_type() -> String {
        "pdd.order.update.address".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
