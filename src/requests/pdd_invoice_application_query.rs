use crate::Request;

use serde::{Deserialize, Serialize};


/// 当消费者在拼多多平台申请开票之后，第三方ERP通过此接口获取开票申请信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddInvoiceApplicationQuery {
    
    /// 订单号；订单号和申请时间必填其一
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 页码，默认1
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 每页返回数目，默认50
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 是否正品发票 0=非正品发票 1=是正品发票
    #[serde(rename = "quality_goods_invoice")]
    pub quality_goods_invoice: Option<i32>,
    
    /// 申请状态：0-已拒绝，1-申请中，2-已同意
    #[serde(rename = "status")]
    pub status: Option<i32>,
    
    /// 申请结束时间, 时间戳（单位毫秒，查询时间间隔不可超过15天）
    #[serde(rename = "update_end_time")]
    pub update_end_time: Option<i64>,
    
    /// 申请开始时间, 时间戳（单位毫秒，查询时间间隔不可超过15天）
    #[serde(rename = "update_start_time")]
    pub update_start_time: Option<i64>,
    
}


impl Request for PddInvoiceApplicationQuery {
    fn get_type() -> String {
        "pdd.invoice.application.query".to_string()
    }

    fn get_response_name() -> String {
        "invoice_application_query_response".to_string()
    }
}
