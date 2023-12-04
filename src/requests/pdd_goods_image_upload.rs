use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品图片上传服务（参数拼接成json串）
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsImageUpload {
    
    /// 支持格式有：jpg/jpeg、png等图片格式，注意入参图片必须转码为base64编码
    #[serde(rename = "image")]
    pub image: Option<String>,
    
}


/// 商品图片上传服务（参数拼接成json串）
impl Request for PddGoodsImageUpload {
    fn get_type() -> String {
        "pdd.goods.image.upload".to_string()
    }

    fn get_response_name() -> String {
        "goods_image_upload_response".to_string()
    }
}
