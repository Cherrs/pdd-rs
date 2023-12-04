use crate::Request;
use crate::PddFile;
use serde::{Deserialize, Serialize};


/// 多多客信息流投放备案视频上传分片上传上传接口，每个分片建议不超过20M
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkReportVideoUploadPart {
    
    /// 当前分片的文件流
    #[serde(rename = "part_file")]
    pub part_file: Option<PddFile>,
    
    /// 当前分片编号名，从1开始
    #[serde(rename = "part_num")]
    pub part_num: Option<String>,
    
    /// 标记本次大文件上传的id（init阶段的返回值）
    #[serde(rename = "upload_sign")]
    pub upload_sign: Option<String>,
    
}


/// 多多客信息流投放备案视频上传分片上传上传接口，每个分片建议不超过20M
impl Request for PddDdkReportVideoUploadPart {
    fn get_type() -> String {
        "pdd.ddk.report.video.upload.part".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
