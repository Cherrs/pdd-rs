use crate::Request;

use serde::{Deserialize, Serialize};


/// 管理尺码表模板时需要删除自定义尺码表模板
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSizespecTemplateDelete {
    
    /// 尺码表模板id
    #[serde(rename = "id")]
    pub id: Option<i64>,
    
}


/// 管理尺码表模板时需要删除自定义尺码表模板
impl Request for PddGoodsSizespecTemplateDelete {
    fn get_type() -> String {
        "pdd.goods.sizespec.template.delete".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
