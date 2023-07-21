
use crate::Request;
use serde::{Deserialize, Serialize};

/// 电子面单云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddWaybillGet {
    /// 入参信息
    #[serde(rename = "param_waybill_cloud_print_apply_new_request")]
    pub param_waybill_cloud_print_apply_new_request: Option<ParamWaybillCloudPrintApplyNewRequest>,
}

/// 电子面单云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Sender {
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

/// 电子面单云打印接口
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

/// 电子面单云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParamWaybillCloudPrintApplyNewRequest {
    /// 设定取号返回的云打印报文是否加密
    #[serde(rename = "need_encrypt")]
    pub need_encrypt: Option<bool>,

    /// 发货人信息
    #[serde(rename = "sender")]
    pub sender: Option<Sender>,

    /// 请求面单信息，数量限制为10
    #[serde(rename = "trade_order_info_dtos")]
    pub trade_order_info_dtos: Option<Vec<TradeOrderInfoDtos>>,

    /// 物流公司Code
    #[serde(rename = "wp_code")]
    pub wp_code: Option<String>,
}

/// 电子面单云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Address {
    /// 城市，仅支持非空值
    #[serde(rename = "city")]
    pub city: Option<String>,

    /// 国家/地区
    #[serde(rename = "country")]
    pub country: Option<String>,

    /// 详细地址，仅支持非空值
    #[serde(rename = "detail")]
    pub detail: Option<String>,

    /// 区，仅支持非空值
    #[serde(rename = "district")]
    pub district: Option<String>,

    /// 省，仅支持非空值
    #[serde(rename = "province")]
    pub province: Option<String>,

    /// 街道
    #[serde(rename = "town")]
    pub town: Option<String>,
}

/// 电子面单云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Items {
    /// 数量
    #[serde(rename = "count")]
    pub count: Option<i32>,

    /// 名称
    #[serde(rename = "name")]
    pub name: Option<String>,
}

/// 电子面单云打印接口
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

    /// 收件人信息
    #[serde(rename = "recipient")]
    pub recipient: Option<Recipient>,

    /// 标准模板模板URL
    #[serde(rename = "template_url")]
    pub template_url: Option<String>,

    /// 使用者ID
    #[serde(rename = "user_id")]
    pub user_id: Option<i64>,
}

/// 电子面单云打印接口
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
    pub total_packages_count: Option<i32>,

    /// 体积, 单位 ml
    #[serde(rename = "volume")]
    pub volume: Option<i64>,

    /// 重量,单位 g
    #[serde(rename = "weight")]
    pub weight: Option<i64>,
}

/// 电子面单云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OrderInfo {
    /// 订单渠道平台编码 拼多多-PDD，淘宝-TB，天猫-TM，京东-JD，阿里巴巴-ALBB，有赞-YZ，微店-WD，蘑菇街-MGJ，云集-YJ，贝贝网-BB，转转-ZZ，快手小店-KS，当当网-DD，小米有品-XMYP，寺库-SK，聚美优品-JM，蜜芽-MY，小红书-XHS，萌推-MT，唯品会-WPH，拍拍-PP，ebay-EBAY，亚马逊-AMAZON，苏宁-SN，国美-GM，1号店-YHD，凡客-VANCL，邮乐-YL，优购-YG，乐蜂-LF，聚尚-JS，拍鞋-PX，银泰-YT，抖音-DY，其他-OTHERS
    #[serde(rename = "order_channels_type")]
    pub order_channels_type: Option<String>,

    /// 订单号,数量限制100
    #[serde(rename = "trade_order_list")]
    pub trade_order_list: Option<Vec<String>>,
}

impl Request for PddWaybillGet {
    fn get_type() -> String {
        "pdd.waybill.get".to_string()
    }

    fn get_response_name() -> String {
        "pdd_waybill_get_response".to_string()
    }
}
