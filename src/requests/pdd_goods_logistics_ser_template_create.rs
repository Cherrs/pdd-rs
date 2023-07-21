use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品送装服务模版新增
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Content {
    
    /// 属性区间大值，-1表示"其他"示例：要配置区间"100-200"的费用，则maxPro输入"200"要配置区间"其他"的费用，则maxPro输入"-1"备注：表示长度时单位为：mm
    #[serde(rename = "max_pro")]
    pub max_pro: Option<i64>,
    
    /// 属性区间小值，-1表示"其他"示例：要配置区间"100-200"的费用，则minPro输入"100"要配置区间"其他"的费用，则minPro输入"-1"备注：表示长度时单位为：mm
    #[serde(rename = "min_pro")]
    pub min_pro: Option<i64>,
    
    /// 价格。单位：分
    #[serde(rename = "price")]
    pub price: Option<i64>,
    
}

/// 商品送装服务模版新增
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct List {
    
    /// 按属性收取费用时配置的内容：按属性限价时设置；按件限价时不用传，value和content必须设置一个
    #[serde(rename = "content")]
    pub content: Option<Vec<Content>>,
    
    /// 分类目收取服务费用方式：1-按件收取费用，2-按属性收取费用
    #[serde(rename = "limit_type")]
    pub limit_type: Option<i32>,
    
    /// 按“件”收取费用时配置的内容：按件限价时输入数字，表示每件商品附加费用$value分；按属性限价时不用传value和content必须设置一个
    #[serde(rename = "value")]
    pub value: Option<i64>,
    
}

/// 商品送装服务模版新增
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServiceAreaList {
    
    /// 市id，如果是全省选中，则市id为0
    #[serde(rename = "city_id")]
    pub city_id: Option<i32>,
    
    /// 区id，如果是全省或全市选中，则区id为0
    #[serde(rename = "district_id")]
    pub district_id: Option<i32>,
    
    /// 省id
    #[serde(rename = "province_id")]
    pub province_id: Option<i32>,
    
    /// 溢价值：按分溢价时，输入价格分的数字，表示value分；按百分比溢价时，输入0-500之间的数字，表示0%——500%备注：买家自提服务类型时，不用传该字段
    #[serde(rename = "value")]
    pub value: Option<i32>,
    
}

/// 商品送装服务模版新增
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsLogisticsSerTemplateCreate {
    
    /// 分类目基础价格配置,入参为string，[{"cat_id4": 1,"cat_id3": 2,"list": [{"limit_type": 1,"value": 1,"content": [{"price": 1,"max_pro": 1,"min_pro": 1}]}]}]
    #[serde(rename = "cat_list")]
    pub cat_list: Option<Vec<CatList>>,
    
    /// 分地区配置溢价时使用的计价单位：0：价格分，按价格分计算费用；1：百分比，按照基础价格乘以百分比计算费用，除了"买家自提"服务，其他服务类型必传
    #[serde(rename = "price_unit")]
    pub price_unit: Option<i32>,
    
    /// 服务地区范围配置，此入参为string,[{"value": 1,"district_id": 2,"city_id": 2,"province_id": 2}]
    #[serde(rename = "service_area_list")]
    pub service_area_list: Option<Vec<ServiceAreaList>>,
    
    /// 服务模板名称（不超过50字）
    #[serde(rename = "template_name")]
    pub template_name: Option<String>,
    
    /// 服务模板类型：2：送货上门 3：送货上门并安装 4：上门安装 5： 买家自提
    #[serde(rename = "template_type")]
    pub template_type: Option<i32>,
    
}

/// 商品送装服务模版新增
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CatList {
    
    /// 三级类目id
    #[serde(rename = "cat_id3")]
    pub cat_id3: Option<i64>,
    
    /// 四级类目id
    #[serde(rename = "cat_id4")]
    pub cat_id4: Option<i64>,
    
    /// 类目规则配置
    #[serde(rename = "list")]
    pub list: Option<Vec<List>>,
    
}


impl Request for PddGoodsLogisticsSerTemplateCreate {
    fn get_type() -> String {
        "pdd.goods.logistics.ser.template.create".to_string()
    }

    fn get_response_name() -> String {
        "goods_logistics_ser_template_create_response".to_string()
    }
}
