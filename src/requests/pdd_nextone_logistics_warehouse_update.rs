use crate::Request;

use serde::{Deserialize, Serialize};


/// 退货入库通知拼多多确认入库成功
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddNextoneLogisticsWarehouseUpdate {
    
    /// request
    #[serde(rename = "request")]
    pub request: Option<PddNextoneLogisticsWarehouseUpdateRequest>,
    
}

/// 退货入库通知拼多多确认入库成功
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddNextoneLogisticsWarehouseUpdateRequest {
    
    /// 售后id
    #[serde(rename = "after_sales_id")]
    pub after_sales_id: Option<i64>,
    
    /// 操作时间
    #[serde(rename = "operate_time")]
    pub operate_time: Option<i64>,
    
    /// order_sn
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 物流公司id
    #[serde(rename = "reverse_logistics_id")]
    pub reverse_logistics_id: Option<i32>,
    
    /// 物流单号
    #[serde(rename = "reverse_tracking_number")]
    pub reverse_tracking_number: Option<String>,
    
    /// 退货入库状态 1：成功；2：失败
    #[serde(rename = "warehouse_status")]
    pub warehouse_status: Option<i32>,
    
}


/// 退货入库通知拼多多确认入库成功
impl Request for PddNextoneLogisticsWarehouseUpdate {
    fn get_type() -> String {
        "pdd.nextone.logistics.warehouse.update".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
