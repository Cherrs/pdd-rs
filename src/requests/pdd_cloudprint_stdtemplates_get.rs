use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取所有标准电子面单模板
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddCloudprintStdtemplatesGet {
    
    /// 快递公司code
    #[serde(rename = "wp_code")]
    pub wp_code: Option<String>,
    
}


/// 获取所有标准电子面单模板
impl Request for PddCloudprintStdtemplatesGet {
    fn get_type() -> String {
        "pdd.cloudprint.stdtemplates.get".to_string()
    }

    fn get_response_name() -> String {
        "pdd_cloudprint_stdtemplates_get_response".to_string()
    }
}
