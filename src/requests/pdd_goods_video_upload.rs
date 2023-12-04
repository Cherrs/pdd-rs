use crate::Request;
use crate::PddFile;
use serde::{Deserialize, Serialize};


/// 商品视频上传接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsVideoUpload {
    
    /// 视频文件,为文件流
    #[serde(rename = "file")]
    pub file: Option<PddFile>,
    
}


/// 商品视频上传接口
impl Request for PddGoodsVideoUpload {
    fn get_type() -> String {
        "pdd.goods.video.upload".to_string()
    }

    fn get_response_name() -> String {
        "goods_video_upload_response".to_string()
    }
}
