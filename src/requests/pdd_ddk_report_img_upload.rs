use crate::Request;
use crate::PddFile;
use serde::{Deserialize, Serialize};


/// 多多客信息流投放备案图片上传
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkReportImgUpload {
    
    /// 多多视频图片文件流
    #[serde(rename = "file")]
    pub file: Option<PddFile>,
    
}


/// 多多客信息流投放备案图片上传
impl Request for PddDdkReportImgUpload {
    fn get_type() -> String {
        "pdd.ddk.report.img.upload".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
