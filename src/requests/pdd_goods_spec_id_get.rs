use crate::Request;

use serde::{Deserialize, Serialize};


/// 生成商家自定义的规格
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSpecIdGet {
    
    /// 拼多多标准规格ID，可以通过pdd.goods.spec.get接口获取
    #[serde(rename = "parent_spec_id")]
    pub parent_spec_id: Option<i64>,
    
    /// 商家编辑的规格值，如颜色规格下设置白色属性
    #[serde(rename = "spec_name")]
    pub spec_name: Option<String>,
    
}


impl Request for PddGoodsSpecIdGet {
    fn get_type() -> String {
        "pdd.goods.spec.id.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_spec_id_get_response".to_string()
    }
}
