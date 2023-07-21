use crate::Request;
use crate::PddFile;
use serde::{Deserialize, Serialize};


/// 商品图片上传
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsImgUpload {
    
    /// 商品图片文件流
    #[serde(rename = "file")]
    pub file: Option<PddFile>,
    
}


impl Request for PddGoodsImgUpload {
    fn get_type() -> String {
        "pdd.goods.img.upload".to_string()
    }

    fn get_response_name() -> String {
        "goods_img_upload_response".to_string()
    }
}
