use crate::Request;

use serde::{Deserialize, Serialize};


/// 供isv使用，获取商家的自定义区的模板信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddCloudprintCustomaresGet {
    
    /// 即pdd.cloudprint.stdtemplates.get接口返回的standard_template_id
    #[serde(rename = "template_id")]
    pub template_id: Option<i32>,
    
}


impl Request for PddCloudprintCustomaresGet {
    fn get_type() -> String {
        "pdd.cloudprint.customares.get".to_string()
    }

    fn get_response_name() -> String {
        "pdd_cloudprint_customares_get_response".to_string()
    }
}
