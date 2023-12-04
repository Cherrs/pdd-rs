use crate::Request;

use serde::{Deserialize, Serialize};


/// 开平侧商家在搬家，根据入参类目+站外属性/站外属性值返回启用的站内外属性映射中的站内属性/站内属性值
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsOutPropertyMappingGet {
    
    /// 拼多多叶子类目id
    #[serde(rename = "cat_id")]
    pub cat_id: Option<i64>,
    
    /// 外部平台属性名称
    #[serde(rename = "out_property_name")]
    pub out_property_name: Option<String>,
    
    /// 外部平台属性值名称
    #[serde(rename = "out_property_value_name")]
    pub out_property_value_name: Option<String>,
    
}


/// 开平侧商家在搬家，根据入参类目+站外属性/站外属性值返回启用的站内外属性映射中的站内属性/站内属性值
impl Request for PddGoodsOutPropertyMappingGet {
    fn get_type() -> String {
        "pdd.goods.out.property.mapping.get".to_string()
    }

    fn get_response_name() -> String {
        "out_property_mapping_get_response".to_string()
    }
}
