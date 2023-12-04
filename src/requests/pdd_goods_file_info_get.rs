use crate::Request;

use serde::{Deserialize, Serialize};


/// 文件详情查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsFileInfoGet {
    
    /// url列表
    #[serde(rename = "url_list")]
    pub url_list: Option<Vec<String>>,
    
}


/// 文件详情查询
impl Request for PddGoodsFileInfoGet {
    fn get_type() -> String {
        "pdd.goods.file.info.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_file_info_response".to_string()
    }
}
