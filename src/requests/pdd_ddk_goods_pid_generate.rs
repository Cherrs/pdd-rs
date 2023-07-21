use crate::Request;

use serde::{Deserialize, Serialize};


/// 创建多多进宝推广位
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkGoodsPidGenerate {
    
    /// 要生成的推广位数量，默认为10，范围为：1~100
    #[serde(rename = "number")]
    pub number: Option<i64>,
    
    /// 推广位名称，例如["1","2"]
    #[serde(rename = "p_id_name_list")]
    pub p_id_name_list: Option<Vec<String>>,
    
    /// 媒体id
    #[serde(rename = "media_id")]
    pub media_id: Option<i64>,
    
}


impl Request for PddDdkGoodsPidGenerate {
    fn get_type() -> String {
        "pdd.ddk.goods.pid.generate".to_string()
    }

    fn get_response_name() -> String {
        "p_id_generate_response".to_string()
    }
}
