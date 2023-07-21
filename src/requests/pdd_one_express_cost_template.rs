use crate::Request;

use serde::{Deserialize, Serialize};


/// 根据id获取拼多多商家的物流运费模板信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOneExpressCostTemplate {
    
    /// 运费模板id
    #[serde(rename = "cost_template_id")]
    pub cost_template_id: Option<i64>,
    
}


impl Request for PddOneExpressCostTemplate {
    fn get_type() -> String {
        "pdd.one.express.cost.template".to_string()
    }

    fn get_response_name() -> String {
        "one_express_cost_template_response".to_string()
    }
}
