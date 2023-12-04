use crate::Request;

use serde::{Deserialize, Serialize};


/// 快递公司处理结果回调接口 附件图片url生成
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsTicketImageUpload {
    
    /// 支持格式有：jpg/jpeg、png等图片格式，入参为图片的base64编码，最大支持1M
    #[serde(rename = "image")]
    pub image: Option<String>,
    
}


/// 快递公司处理结果回调接口 附件图片url生成
impl Request for PddLogisticsTicketImageUpload {
    fn get_type() -> String {
        "pdd.logistics.ticket.image.upload".to_string()
    }

    fn get_response_name() -> String {
        "logistics_ticket_image_upload_response".to_string()
    }
}
