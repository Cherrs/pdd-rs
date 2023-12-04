use crate::Request;

use serde::{Deserialize, Serialize};


/// 使用商家订单上的收件人信息电子面单取号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PackageInfo {
    
    /// 快运货品描述
    #[serde(rename = "goods_description")]
    pub goods_description: Option<String>,
    
    /// 包裹id,拆合单使用
    #[serde(rename = "id")]
    pub id: Option<String>,
    
    /// 商品信息,数量限制为100
    #[serde(rename = "items")]
    pub items: Option<Vec<Items>>,
    
    /// 快运包装方式描述
    #[serde(rename = "packaging_description")]
    pub packaging_description: Option<String>,
    
    /// 子母件总包裹数
    #[serde(rename = "total_packages_count")]
    pub total_packages_count: Option<String>,
    
    /// 体积, 单位 ml
    #[serde(rename = "volume")]
    pub volume: Option<i32>,
    
    /// 重量,单位 g
    #[serde(rename = "weight")]
    pub weight: Option<i32>,
    
}

/// 使用商家订单上的收件人信息电子面单取号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Sender {
    
    /// 发货地址，需要入参与 search 接口中的发货人地址信息一致
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

/// 使用商家订单上的收件人信息电子面单取号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParamFdsWaybillGetRequest {
    
    /// 发货人信息
    #[serde(rename = "sender")]
    pub sender: Option<Sender>,
    
    /// 取号列表
    #[serde(rename = "trade_order_info_dtos")]
    pub trade_order_info_dtos: Option<Vec<TradeOrderInfoDtos>>,
    
    /// 物流公司 Code ，枚举： YTO- 圆通，ZTO-中通，YUNDA-韵达，STO-申通
    #[serde(rename = "wp_code")]
    pub wp_code: Option<String>,
    
}

/// 使用商家订单上的收件人信息电子面单取号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFdsWaybillGet {
    
    /// 入参信息
    #[serde(rename = "param_fds_waybill_get_request")]
    pub param_fds_waybill_get_request: Option<ParamFdsWaybillGetRequest>,
    
}

/// 使用商家订单上的收件人信息电子面单取号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Address {
    
    /// 市
    #[serde(rename = "city")]
    pub city: Option<String>,
    
    /// 国家/地区
    #[serde(rename = "country")]
    pub country: Option<String>,
    
    /// 详细地址
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    
    /// 区
    #[serde(rename = "district")]
    pub district: Option<String>,
    
    /// 省
    #[serde(rename = "province")]
    pub province: Option<String>,
    
    /// 街道
    #[serde(rename = "town")]
    pub town: Option<String>,
    
}

/// 使用商家订单上的收件人信息电子面单取号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OrderInfo {
    
    /// 订单渠道平台编码
    #[serde(rename = "order_channels_type")]
    pub order_channels_type: Option<String>,
    
    /// 订单列表，限制100个
    #[serde(rename = "trade_order_list")]
    pub trade_order_list: Option<Vec<TradeOrderList>>,
    
}

/// 使用商家订单上的收件人信息电子面单取号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TradeOrderList {
    
    /// 代打店铺id
    #[serde(rename = "mall_mask_id")]
    pub mall_mask_id: Option<String>,
    
    /// 代打订单号
    #[serde(rename = "order_mask_sn")]
    pub order_mask_sn: Option<String>,
    
}

/// 使用商家订单上的收件人信息电子面单取号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TradeOrderInfoDtos {
    
    /// 物流服务内容链接
    #[serde(rename = "logistics_services")]
    pub logistics_services: Option<String>,
    
    /// 请求id
    #[serde(rename = "object_id")]
    pub object_id: Option<String>,
    
    /// 订单信息
    #[serde(rename = "order_info")]
    pub order_info: Option<OrderInfo>,
    
    /// 包裹信息
    #[serde(rename = "package_info")]
    pub package_info: Option<PackageInfo>,
    
    /// 标准模板模板URL
    #[serde(rename = "template_url")]
    pub template_url: Option<String>,
    
    /// 使用者ID
    #[serde(rename = "user_id")]
    pub user_id: Option<i64>,
    
}

/// 使用商家订单上的收件人信息电子面单取号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Items {
    
    /// 数量
    #[serde(rename = "count")]
    pub count: Option<i32>,
    
    /// 商品名称
    #[serde(rename = "name")]
    pub name: Option<String>,
    
}


/// 使用商家订单上的收件人信息电子面单取号
impl Request for PddFdsWaybillGet {
    fn get_type() -> String {
        "pdd.fds.waybill.get".to_string()
    }

    fn get_response_name() -> String {
        "pdd_fds_waybill_get_response".to_string()
    }
}
