use crate::Request;

use serde::{Deserialize, Serialize};


/// 快递公司处理结果回调
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsTicketNotify {
    
    /// 附件url,示例：["http://testimg.yangkeduo.com/pdd_oms/2018-01-16/411068e948835ae053a86c13f8ebb5ee.jpg"]
    #[serde(rename = "attach_path_list")]
    pub attach_path_list: Option<Vec<String>>,
    
    /// 工单id
    #[serde(rename = "ticket_id")]
    pub ticket_id: Option<i64>,
    
    /// 运单号
    #[serde(rename = "waybill_no")]
    pub waybill_no: Option<String>,
    
    /// 处理结果
    #[serde(rename = "handle_result")]
    pub handle_result: Option<String>,
    
    /// 签收状态，0:默认,1:未签收,2:已签收
    #[serde(rename = "sign_state")]
    pub sign_state: Option<i32>,
    
    /// 是否赔付，0:默认,1:未赔付,2:已赔付
    #[serde(rename = "compensate_state")]
    pub compensate_state: Option<i32>,
    
    /// 赔付金额(单位:分)
    #[serde(rename = "compensate_amount")]
    pub compensate_amount: Option<i64>,
    
    /// 责任方，0:默认, 1:消费者,2:商家,3:快递公司,4:其他
    #[serde(rename = "duty")]
    pub duty: Option<i32>,
    
    /// 处理人
    #[serde(rename = "express_dealer")]
    pub express_dealer: Option<String>,
    
    /// 处理人联系方式
    #[serde(rename = "express_dealer_contact")]
    pub express_dealer_contact: Option<String>,
    
}


impl Request for PddLogisticsTicketNotify {
    fn get_type() -> String {
        "pdd.logistics.ticket.notify".to_string()
    }

    fn get_response_name() -> String {
        "logistics_ticket_notify_response".to_string()
    }
}
