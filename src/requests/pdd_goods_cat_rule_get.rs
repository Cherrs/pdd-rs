use crate::Request;

use serde::{Deserialize, Serialize};


/// 通过叶子类目id获取该类目的发布规则，目前返回标品、商品服务、属性等规则。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsCatRuleGet {
    
    /// 类目id
    #[serde(rename = "cat_id")]
    pub cat_id: Option<i64>,
    
    /// 商品id，编辑的时候需要传被编辑的商品id，发布商品时如果已有商品id也需要传
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddGoodsCatRuleGet {
    fn get_type() -> String {
        "pdd.goods.cat.rule.get".to_string()
    }

    fn get_response_name() -> String {
        "cat_rule_get_response".to_string()
    }
}
