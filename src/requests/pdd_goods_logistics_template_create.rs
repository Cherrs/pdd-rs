use crate::Request;

use serde::{Deserialize, Serialize};


/// 创建物流模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CostProvinceList {
    
    /// 省份ID
    #[serde(rename = "province_id")]
    pub province_id: Option<i32>,
    
}

/// 创建物流模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsLogisticsTemplateCreate {
    
    /// 非包邮模版列表
    #[serde(rename = "cost_template_list")]
    pub cost_template_list: Option<Vec<CostTemplateList>>,
    
    /// 包邮地区
    #[serde(rename = "free_province_list")]
    pub free_province_list: Option<Vec<FreeProvinceList>>,
    
    /// 计费方式，0-按件计费，1-按重量计费
    #[serde(rename = "cost_type")]
    pub cost_type: Option<i32>,
    
    /// 运费模板名称
    #[serde(rename = "template_name")]
    pub template_name: Option<String>,
    
    /// 发货地省份id
    #[serde(rename = "province_id")]
    pub province_id: Option<i32>,
    
    /// 发货地城市id
    #[serde(rename = "city_id")]
    pub city_id: Option<i32>,
    
    /// 发货地区id
    #[serde(rename = "district_id")]
    pub district_id: Option<i32>,
    
}

/// 创建物流模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CostTemplateList {
    
    /// 首件
    #[serde(rename = "first_standard")]
    pub first_standard: Option<i64>,
    
    /// 首件或首重价格，单位为分
    #[serde(rename = "first_cost")]
    pub first_cost: Option<i64>,
    
    /// 续重或续件，续重时单位为克且数值须为1000的整数倍
    #[serde(rename = "add_standard")]
    pub add_standard: Option<i64>,
    
    /// 续件或续重价格，单位为分
    #[serde(rename = "add_cost")]
    pub add_cost: Option<i64>,
    
    /// 对不包邮地区，true-若要包邮须满足件数包邮，false-不开启满足件数包邮
    #[serde(rename = "is_have_free_min_count")]
    pub is_have_free_min_count: Option<bool>,
    
    /// 对不包邮地区，满足指定件数包邮，该值为商家设置的指定件数，若为-1则商家没有开启满足件数包邮
    #[serde(rename = "have_free_min_count")]
    pub have_free_min_count: Option<i32>,
    
    /// 对不包邮地区，true-若要包邮须满足指定价格则可以包邮，false-不开启满足指定价格包邮
    #[serde(rename = "is_have_free_min_amount")]
    pub is_have_free_min_amount: Option<bool>,
    
    /// 对不包邮地区，满足指定价格包邮，该值为商家设置的指定订单金额，若为-1则商家没有开启满足指定价格包邮，注意，单位为分
    #[serde(rename = "have_free_min_amount")]
    pub have_free_min_amount: Option<i64>,
    
    /// 省份列表
    #[serde(rename = "cost_province_list")]
    pub cost_province_list: Option<Vec<CostProvinceList>>,
    
}

/// 创建物流模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FreeProvinceList {
    
    /// 省份ID
    #[serde(rename = "province_id")]
    pub province_id: Option<i32>,
    
}


impl Request for PddGoodsLogisticsTemplateCreate {
    fn get_type() -> String {
        "pdd.goods.logistics.template.create".to_string()
    }

    fn get_response_name() -> String {
        "goods_logistics_template_create_response".to_string()
    }
}
