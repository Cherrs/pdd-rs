use crate::Request;
use crate::PddFile;
use serde::{Deserialize, Serialize};


/// 多多客信息流投放备案视频上传,上传视频大小有限制,单个文件超过20M需要走分片上传
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkReportVideoUpload {
    
    /// 多多客信息流投放备案视频文件流
    #[serde(rename = "file")]
    pub file: Option<PddFile>,
    
}


impl Request for PddDdkReportVideoUpload {
    fn get_type() -> String {
        "pdd.ddk.report.video.upload".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
