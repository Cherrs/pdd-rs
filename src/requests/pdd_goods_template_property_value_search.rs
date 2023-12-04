use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品发布过程中，填写商品属性时，用于模糊搜索属性模板上可填属性值
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsTemplatePropertyValueSearch {
    
    /// 类目id
    #[serde(rename = "cat_id")]
    pub cat_id: Option<i64>,
    
    /// 页码 从1开始
    #[serde(rename = "page_num")]
    pub page_num: Option<i32>,
    
    /// 1页查询的最大数据 [1，500], 默认100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 父属性值id
    #[serde(rename = "parent_vid")]
    pub parent_vid: Option<i64>,
    
    /// 模板属性id，废弃中，请入参属性id
    #[serde(rename = "template_pid")]
    pub template_pid: Option<i64>,
    
    /// 需要模糊搜索的属性值
    #[serde(rename = "value")]
    pub value: Option<String>,
    
    /// 属性id
    #[serde(rename = "ref_pid")]
    pub ref_pid: Option<i64>,
    
}


/// 商品发布过程中，填写商品属性时，用于模糊搜索属性模板上可填属性值
impl Request for PddGoodsTemplatePropertyValueSearch {
    fn get_type() -> String {
        "pdd.goods.template.property.value.search".to_string()
    }

    fn get_response_name() -> String {
        "goods_template_prop_val_search_response".to_string()
    }
}
