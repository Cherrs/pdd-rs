use crate::Request;

use serde::{Deserialize, Serialize};


/// 海淘服务商上传商品备案信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddCustomsSendGoodsRecordRequest {
    
    /// 备案商品列表
    #[serde(rename = "goods_list")]
    pub goods_list: Option<Vec<GoodsList>>,
    
}

/// 海淘服务商上传商品备案信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddCustomsSendGoodsRecord {
    
    /// 上传备案商品请求
    #[serde(rename = "request")]
    pub request: Option<PddCustomsSendGoodsRecordRequest>,
    
}

/// 海淘服务商上传商品备案信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GoodsList {
    
    /// 条形码
    #[serde(rename = "bar_code")]
    pub bar_code: Option<String>,
    
    /// 保税仓名称
    #[serde(rename = "bonded_warehouse_name")]
    pub bonded_warehouse_name: Option<String>,
    
    /// 品牌中文名称
    #[serde(rename = "brand_chinese_name")]
    pub brand_chinese_name: Option<String>,
    
    /// 品牌英文名称
    #[serde(rename = "brand_english_name")]
    pub brand_english_name: Option<String>,
    
    /// 品类
    #[serde(rename = "category")]
    pub category: Option<String>,
    
    /// 消费税率，单位%
    #[serde(rename = "consumption_tax_rate")]
    pub consumption_tax_rate: Option<f32>,
    
    /// 成本价（RMB）
    #[serde(rename = "cost_price")]
    pub cost_price: Option<f32>,
    
    /// 海关关区代码
    #[serde(rename = "customs_code")]
    pub customs_code: Option<String>,
    
    /// 备案电商企业的海关注册登记名称(备案的电商企业名称)
    #[serde(rename = "ebc_name")]
    pub ebc_name: Option<String>,
    
    /// 账册编号
    #[serde(rename = "ems_no")]
    pub ems_no: Option<String>,
    
    /// 保质期
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<String>,
    
    /// 毛重（KG）
    #[serde(rename = "gross_weight")]
    pub gross_weight: Option<f32>,
    
    /// 海关HS code
    #[serde(rename = "hs_code")]
    pub hs_code: Option<String>,
    
    /// 备案商品图片链接
    #[serde(rename = "img_url")]
    pub img_url: Option<String>,
    
    /// 电商企业的商品编号(skuId非pdd skuId)
    #[serde(rename = "item_no")]
    pub item_no: Option<String>,
    
    /// 物料号
    #[serde(rename = "item_record_no")]
    pub item_record_no: Option<String>,
    
    /// 生产企业名称
    #[serde(rename = "manufacturing_company_name")]
    pub manufacturing_company_name: Option<String>,
    
    /// 生产企业注册号
    #[serde(rename = "manufacturing_company_registration_no")]
    pub manufacturing_company_registration_no: Option<String>,
    
    /// 生产厂家地址（奶制品必填）
    #[serde(rename = "manufacturing_factory_address")]
    pub manufacturing_factory_address: Option<String>,
    
    /// 净重（KG）
    #[serde(rename = "net_weight")]
    pub net_weight: Option<f32>,
    
    /// 海关口岸代码
    #[serde(rename = "port_code")]
    pub port_code: Option<String>,
    
    /// 生产国代码
    #[serde(rename = "producing_country")]
    pub producing_country: Option<String>,
    
    /// 产品国检备案编号
    #[serde(rename = "product_record_no")]
    pub product_record_no: Option<String>,
    
    /// 法定第一数量
    #[serde(rename = "qty1")]
    pub qty1: Option<f32>,
    
    /// 法定第二数量
    #[serde(rename = "qty2")]
    pub qty2: Option<f32>,
    
    /// 备案商品中文名称
    #[serde(rename = "record_chinese_name")]
    pub record_chinese_name: Option<String>,
    
    /// 备案商品英文名称
    #[serde(rename = "record_english_name")]
    pub record_english_name: Option<String>,
    
    /// 商品规格型号(报文gmodel)
    #[serde(rename = "record_model")]
    pub record_model: Option<String>,
    
    /// 型号
    #[serde(rename = "specification")]
    pub specification: Option<String>,
    
    /// 库存数量
    #[serde(rename = "stock")]
    pub stock: Option<i64>,
    
    /// 库存时间
    #[serde(rename = "stock_time")]
    pub stock_time: Option<String>,
    
    /// 关税税率,单位%
    #[serde(rename = "tariff_rate")]
    pub tariff_rate: Option<f32>,
    
    /// 申报单位代码
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    
    /// 法定第一单位代码
    #[serde(rename = "unit1")]
    pub unit1: Option<String>,
    
    /// 法定第二单位代码
    #[serde(rename = "unit2")]
    pub unit2: Option<String>,
    
    /// 单价（RMB）
    #[serde(rename = "unit_price")]
    pub unit_price: Option<f32>,
    
    /// 增值税率，单位%
    #[serde(rename = "value_added_tax_rate")]
    pub value_added_tax_rate: Option<f32>,
    
    /// 供应商名称
    #[serde(rename = "vendor_name")]
    pub vendor_name: Option<String>,
    
    /// 备案仓储企业代码
    #[serde(rename = "wc_code")]
    pub wc_code: Option<String>,
    
    /// 备案仓储企业的海关注册登记名称
    #[serde(rename = "wc_name")]
    pub wc_name: Option<String>,
    
    /// 网络链接
    #[serde(rename = "website")]
    pub website: Option<String>,
    
    /// 包装方式
    #[serde(rename = "wrap_type")]
    pub wrap_type: Option<String>,
    
}


impl Request for PddCustomsSendGoodsRecord {
    fn get_type() -> String {
        "pdd.customs.send.goods.record".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
