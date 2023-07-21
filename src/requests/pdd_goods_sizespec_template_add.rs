use crate::Request;

use serde::{Deserialize, Serialize};


/// 管理尺码表模板时需要新增自定义尺码表模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Values {
    
    /// 尺码组和尺码表元素的id
    #[serde(rename = "$key")]
    pub key: Option<i32>,
    
    /// 尺码组和尺码表元素的值
    #[serde(rename = "$value")]
    pub value: Option<String>,
    
}

/// 管理尺码表模板时需要新增自定义尺码表模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Groups {
    
    /// 尺码元数据id
    #[serde(rename = "id")]
    pub id: Option<i32>,
    
    /// 尺码元数据名称
    #[serde(rename = "name")]
    pub name: Option<String>,
    
}

/// 管理尺码表模板时需要新增自定义尺码表模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SizeSpecDto {
    
    /// 尺码表分类id，pdd.goods.sizespec.class.get得到
    #[serde(rename = "class_id")]
    pub class_id: Option<i32>,
    
    /// 尺码表内容
    #[serde(rename = "content")]
    pub content: Option<Content>,
    
    /// 尺码表名称
    #[serde(rename = "name")]
    pub name: Option<String>,
    
}

/// 管理尺码表模板时需要新增自定义尺码表模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Meta {
    
    /// 尺码元素
    #[serde(rename = "elements")]
    pub elements: Option<Vec<Elements>>,
    
    /// 尺码组
    #[serde(rename = "groups")]
    pub groups: Option<Vec<Groups>>,
    
}

/// 管理尺码表模板时需要新增自定义尺码表模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSizespecTemplateAdd {
    
    /// 尺码表
    #[serde(rename = "size_spec_dto")]
    pub size_spec_dto: Option<SizeSpecDto>,
    
}

/// 管理尺码表模板时需要新增自定义尺码表模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Elements {
    
    /// 尺码元数据id
    #[serde(rename = "id")]
    pub id: Option<i32>,
    
    /// 尺码元数据名称
    #[serde(rename = "name")]
    pub name: Option<String>,
    
}

/// 管理尺码表模板时需要新增自定义尺码表模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Records {
    
    /// 尺码组和尺码表元素的值
    #[serde(rename = "values")]
    pub values: Option<Values>,
    
}

/// 管理尺码表模板时需要新增自定义尺码表模版
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Content {
    
    /// 尺码表元数据（表头），pdd.goods.sizespec.meta.get得到
    #[serde(rename = "meta")]
    pub meta: Option<Meta>,
    
    /// 尺码表行数据
    #[serde(rename = "records")]
    pub records: Option<Vec<Records>>,
    
}


impl Request for PddGoodsSizespecTemplateAdd {
    fn get_type() -> String {
        "pdd.goods.sizespec.template.add".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
