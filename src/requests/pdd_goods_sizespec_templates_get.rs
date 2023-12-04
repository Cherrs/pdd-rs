use crate::Request;

use serde::{Deserialize, Serialize};


/// 管理尺码表模板时需要查询尺码表模板列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSizespecTemplatesGet {
    
    /// 尺码表分类id，pdd.goods.sizespec.class.get得到
    #[serde(rename = "class_id")]
    pub class_id: Option<i64>,
    
    /// 限制数量
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    
    /// 偏移量
    #[serde(rename = "offset")]
    pub offset: Option<i64>,
    
}


/// 管理尺码表模板时需要查询尺码表模板列表
impl Request for PddGoodsSizespecTemplatesGet {
    fn get_type() -> String {
        "pdd.goods.sizespec.templates.get".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
