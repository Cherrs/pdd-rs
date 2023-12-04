use crate::Request;

use serde::{Deserialize, Serialize};


/// 尺码表分类查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSizespecClassGet {
    
}


/// 管理尺码表模板、创建尺码表模板需要使用分类管理
impl Request for PddGoodsSizespecClassGet {
    fn get_type() -> String {
        "pdd.goods.sizespec.class.get".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
