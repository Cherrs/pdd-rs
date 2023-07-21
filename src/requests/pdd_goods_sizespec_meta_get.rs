use crate::Request;

use serde::{Deserialize, Serialize};


/// 创建尺码表需要查询尺码表分类支持的尺码组和尺码参数（元数据）
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSizespecMetaGet {
    
    /// 尺码分类id
    #[serde(rename = "class_id")]
    pub class_id: Option<i32>,
    
}


impl Request for PddGoodsSizespecMetaGet {
    fn get_type() -> String {
        "pdd.goods.sizespec.meta.get".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
