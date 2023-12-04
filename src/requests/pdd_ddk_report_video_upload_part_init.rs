use crate::Request;

use serde::{Deserialize, Serialize};


/// 多多客信息流投放备案视频上传分片初始化
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkReportVideoUploadPartInit {
    
    /// 文件对应的contentType，且必须为视频类型
    #[serde(rename = "content_type")]
    pub content_type: Option<String>,
    
}


/// 多多客信息流投放备案视频上传分片初始化
impl Request for PddDdkReportVideoUploadPartInit {
    fn get_type() -> String {
        "pdd.ddk.report.video.upload.part.init".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
