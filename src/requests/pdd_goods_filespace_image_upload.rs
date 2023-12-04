use crate::Request;
use crate::PddFile;
use serde::{Deserialize, Serialize};


/// 图片上传到图片空间
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsFilespaceImageUpload {
    
    /// 图片文件流
    #[serde(rename = "file")]
    pub file: Option<PddFile>,
    
}


/// 图片上传到图片空间
impl Request for PddGoodsFilespaceImageUpload {
    fn get_type() -> String {
        "pdd.goods.filespace.image.upload".to_string()
    }

    fn get_response_name() -> String {
        "goods_filespace_image_upload_response".to_string()
    }
}
