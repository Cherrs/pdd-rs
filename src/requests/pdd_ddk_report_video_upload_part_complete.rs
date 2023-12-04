use crate::Request;

use serde::{Deserialize, Serialize};


/// 多多客信息流投放备案视频上传分片完成
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkReportVideoUploadPartComplete {
    
    /// 标记本次大文件上传的id（init阶段的返回值）
    #[serde(rename = "upload_sign")]
    pub upload_sign: Option<String>,
    
}


/// 多多客信息流投放备案视频上传分片完成
impl Request for PddDdkReportVideoUploadPartComplete {
    fn get_type() -> String {
        "pdd.ddk.report.video.upload.part.complete".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
