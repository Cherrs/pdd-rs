use crate::Request;

use serde::{Deserialize, Serialize};


/// 上传白底图长图等素材到商品
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsMaterialCreate {
    
    /// 素材内容（一般为图片链接）
    #[serde(rename = "content")]
    pub content: Option<String>,
    
    /// 图片空间文件id
    #[serde(rename = "file_id")]
    pub file_id: Option<i64>,
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 素材类型（1：白底图；3：长图）
    #[serde(rename = "material_type")]
    pub material_type: Option<i32>,
    
}


/// 上传白底图长图等素材到商品
impl Request for PddGoodsMaterialCreate {
    fn get_type() -> String {
        "pdd.goods.material.create".to_string()
    }

    fn get_response_name() -> String {
        "error_code".to_string()
    }
}
