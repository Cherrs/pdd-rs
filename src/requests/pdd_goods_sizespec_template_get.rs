use crate::Request;

use serde::{Deserialize, Serialize};


/// 管理尺码表模板时需要单独查询尺码表模板
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSizespecTemplateGet {
    
    /// 尺码表id
    #[serde(rename = "id")]
    pub id: Option<i64>,
    
}


impl Request for PddGoodsSizespecTemplateGet {
    fn get_type() -> String {
        "pdd.goods.sizespec.template.get".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
