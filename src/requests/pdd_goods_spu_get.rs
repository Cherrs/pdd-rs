use crate::Request;

use serde::{Deserialize, Serialize};


/// 根据标品类目和关键属性获取标品详情信息，可以先通过pdd.goods.spu.search接口获取标品的类目及关键属性。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KeyProp {
    
    /// 引用属性ID
    #[serde(rename = "ref_pid")]
    pub ref_pid: Option<i64>,
    
    /// 属性值单位
    #[serde(rename = "value_unit")]
    pub value_unit: Option<String>,
    
    /// 关键属性值，和vid必须入参其一。
    #[serde(rename = "value")]
    pub value: Option<String>,
    
    /// 关键属性值ID，和value必须入参其一。
    #[serde(rename = "vid")]
    pub vid: Option<i64>,
    
}

/// 根据标品类目和关键属性获取标品详情信息，可以先通过pdd.goods.spu.search接口获取标品的类目及关键属性。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSpuGet {
    
    /// 标品所在的类目ID
    #[serde(rename = "cat_id")]
    pub cat_id: Option<String>,
    
    /// 关键属性
    #[serde(rename = "key_prop")]
    pub key_prop: Option<Vec<KeyProp>>,
    
}


/// 根据标品类目和关键属性获取标品详情信息，可以先通过pdd.goods.spu.search接口获取标品的类目及关键属性。
impl Request for PddGoodsSpuGet {
    fn get_type() -> String {
        "pdd.goods.spu.get".to_string()
    }

    fn get_response_name() -> String {
        "spu_get_response".to_string()
    }
}
