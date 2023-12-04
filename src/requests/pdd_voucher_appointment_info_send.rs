use crate::Request;

use serde::{Deserialize, Serialize};


/// 第三方ISV将消费者的预约提货信息同步给平台
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVoucherAppointmentInfoSend {
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 外部流水号
    #[serde(rename = "out_biz_no")]
    pub out_biz_no: Option<String>,
    
    /// 优惠券信息列表,例子[{"voucher_id":"test voucher_id","voucher_no":"test voucher_no"}]
    #[serde(rename = "voucher_list")]
    pub voucher_list: Option<Vec<VoucherList>>,
    
    /// 物流方式  1  物流发货   2 自提
    #[serde(rename = "logistics_type")]
    pub logistics_type: Option<i32>,
    
    /// 预约时间, 距离格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总毫秒数
    #[serde(rename = "appointment_time")]
    pub appointment_time: Option<i64>,
    
}

/// 第三方ISV将消费者的预约提货信息同步给平台
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VoucherList {
    
    /// 卡券ID
    #[serde(rename = "voucher_id")]
    pub voucher_id: Option<String>,
    
    /// 卡券号
    #[serde(rename = "voucher_no")]
    pub voucher_no: Option<String>,
    
}


/// 第三方ISV将消费者的预约提货信息同步给平台
impl Request for PddVoucherAppointmentInfoSend {
    fn get_type() -> String {
        "pdd.voucher.appointment.info.send".to_string()
    }

    fn get_response_name() -> String {
        "voucher_appointment_info_send_response".to_string()
    }
}
