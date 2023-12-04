use crate::Request;

use serde::{Deserialize, Serialize};


/// 供应商向拼多多进行创单回调请求
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tickets {
    
    /// 辅助凭证，有辅助凭证时返回
    #[serde(rename = "additional")]
    pub additional: Option<String>,
    
    /// 主凭证，code_type=2时返回
    #[serde(rename = "code")]
    pub code: Option<String>,
    
    /// 文件base64流，code_type=3时返回，大小小于800KB
    #[serde(rename = "file")]
    pub file: Option<String>,
    
    /// 外链，code_type=4时返回
    #[serde(rename = "url")]
    pub url: Option<String>,
    
}

/// 供应商向拼多多进行创单回调请求
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTicketOrderCreateNotifycation {
    
    /// 码类型。status=2时必填。1.无凭证(身份证/手机号) 2. 数字码 3.QR图片 4.外链
    #[serde(rename = "code_type")]
    pub code_type: Option<i32>,
    
    /// 失败错误码。status=3时必填
    #[serde(rename = "failed_code")]
    pub failed_code: Option<i32>,
    
    /// 失败原因。 status=3时必填
    #[serde(rename = "failed_reason")]
    pub failed_reason: Option<String>,
    
    /// 拼多多制票号
    #[serde(rename = "order_no")]
    pub order_no: Option<String>,
    
    /// isv订单号
    #[serde(rename = "out_order_sn")]
    pub out_order_sn: Option<String>,
    
    /// 制码状态。 2.制作成功 3.制作失败
    #[serde(rename = "status")]
    pub status: Option<i32>,
    
    /// 凭证信息列表。status=2 且 code_type!=1 时必填
    #[serde(rename = "tickets")]
    pub tickets: Option<Vec<Tickets>>,
    
    /// 凭证类型。status=2时必填。1.一人一码 2.一人多码
    #[serde(rename = "ticket_type")]
    pub ticket_type: Option<i32>,
    
}


/// 供应商向拼多多进行创单回调请求
impl Request for PddTicketOrderCreateNotifycation {
    fn get_type() -> String {
        "pdd.ticket.order.create.notifycation".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
