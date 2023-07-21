use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品履约生效规则查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTicketSkuRuleGet {
    
    /// 商户履约规则 id
    #[serde(rename = "out_rule_id")]
    pub out_rule_id: Option<String>,
    
    /// 上传商品的上传序列 ID
    #[serde(rename = "rule_id")]
    pub rule_id: Option<String>,
    
}


impl Request for PddTicketSkuRuleGet {
    fn get_type() -> String {
        "pdd.ticket.sku.rule.get".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
