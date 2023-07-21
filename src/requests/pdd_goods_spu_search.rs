use crate::Request;

use serde::{Deserialize, Serialize};


/// 可以通过标品名称或者类目+关键属性的值两种模式进行搜索，搜索到的标品需要在标品详情接口获取详细信息。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KeyProp {
    
    /// 关键属性的引用属性ID，需要从pdd.cat.rule.get中获取。
    #[serde(rename = "ref_pid")]
    pub ref_pid: Option<i64>,
    
    /// 关键属性值，需要从pdd.goods.cat.rule.get中获取。当要根据关键属性匹配时，和vid必须入参其一。
    #[serde(rename = "value")]
    pub value: Option<String>,
    
    /// 属性值单位
    #[serde(rename = "value_unit")]
    pub value_unit: Option<String>,
    
    /// 关键属性值ID，需要从pdd.goods.cat.rule.get中获取规则。当要根据关键属性匹配时，和value必须入参其一。
    #[serde(rename = "vid")]
    pub vid: Option<i64>,
    
}

/// 可以通过标品名称或者类目+关键属性的值两种模式进行搜索，搜索到的标品需要在标品详情接口获取详细信息。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSpuSearch {
    
    /// 类目ID，可以是一二三四级类目，在该类目下进行搜索。
    #[serde(rename = "cat_id")]
    pub cat_id: Option<i64>,
    
    /// 标品关键属性精确匹配。和标品标题必须入参其一。
    #[serde(rename = "key_prop")]
    pub key_prop: Option<Vec<KeyProp>>,
    
    /// 标品标题模糊搜索。和关键属性必须入参其一。
    #[serde(rename = "spu_name")]
    pub spu_name: Option<String>,
    
}


impl Request for PddGoodsSpuSearch {
    fn get_type() -> String {
        "pdd.goods.spu.search".to_string()
    }

    fn get_response_name() -> String {
        "spu_search_response".to_string()
    }
}
