use crate::Request;

use serde::{Deserialize, Serialize};


/// 尺码表分类查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSizespecClassGet {
    
}


impl Request for PddGoodsSizespecClassGet {
    fn get_type() -> String {
        "pdd.goods.sizespec.class.get".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
