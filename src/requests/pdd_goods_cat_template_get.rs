use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品发布前，需要查询该类目的商品发布需要的属性，获取商品发布需要的模板-属性-属性值。已废弃，建议使用pdd.goods.cat.rule.get代替
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCatTemplateGet {
    
    /// 类目id
    #[serde(rename = "cat_id")]
    pub cat_id: Option<i64>,
    
}


/// 商品发布前，需要查询该类目的商品发布需要的属性，获取商品发布需要的模板-属性-属性值。已废弃，建议使用pdd.goods.cat.rule.get代替
impl Request for PddGoodsCatTemplateGet {
    fn get_type() -> String {
        "pdd.goods.cat.template.get".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
