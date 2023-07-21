use crate::Request;

use serde::{Deserialize, Serialize};


/// 跨境全托管发货单有物流发货
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsFulfillmentSend {
    
    /// 发货单号
    #[serde(rename = "fulfillment_sn")]
    pub fulfillment_sn: Option<String>,
    
    /// 快递公司编号
    #[serde(rename = "logistics_id")]
    pub logistics_id: Option<i32>,
    
    /// 枚举：1=首次发货：用于托管发货单首次发货，仅待发货托管单可传入； 2=修改发货：用于修改发货，调用成功后将会覆盖原发货信息，仅已发货2小时内的托管发货单可传入。若不传入该字段，系统将默认为首次发货。
    #[serde(rename = "redelivery_type")]
    pub redelivery_type: Option<i32>,
    
    /// 退货地址id（可通过pdd.refund.address.list.get获取）
    #[serde(rename = "return_address_id")]
    pub return_address_id: Option<String>,
    
    /// 发货运单号
    #[serde(rename = "tracking_number")]
    pub tracking_number: Option<String>,
    
}


impl Request for PddLogisticsFulfillmentSend {
    fn get_type() -> String {
        "pdd.logistics.fulfillment.send".to_string()
    }

    fn get_response_name() -> String {
        "logistics_send_response".to_string()
    }
}
