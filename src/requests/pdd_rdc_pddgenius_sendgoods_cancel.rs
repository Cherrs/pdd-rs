use crate::Request;

use serde::{Deserialize, Serialize};


/// 用于未发货仅退款服务商通知拼多多PG取消成功
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Param {
    
    /// 错误码：1001 错误场景：该订单未同步，无法取消发货 实际含义：订单未同步到isv; 错误码：1002 错误场景：该订单已发货，无法取消发货 实际含义：订单已经发货; 错误码：1003 错误场景：该订单已发货，无法取消发货 实际含义：订单已打印电子面单
    #[serde(rename = "fail_reason_code")]
    pub fail_reason_code: Option<i32>,
    
    /// 描述
    #[serde(rename = "msg")]
    pub msg: Option<String>,
    
    /// 操作时间戳（毫秒）
    #[serde(rename = "operate_time")]
    pub operate_time: Option<i64>,
    
    /// 退款金额 单位 分
    #[serde(rename = "refund_fee")]
    pub refund_fee: Option<i32>,
    
    /// 退款单ID
    #[serde(rename = "refund_id")]
    pub refund_id: Option<i64>,
    
    /// 状态SUCCESS、FAIL
    #[serde(rename = "status")]
    pub status: Option<String>,
    
    /// 订单号
    #[serde(rename = "tid")]
    pub tid: Option<String>,
    
}

/// 用于未发货仅退款服务商通知拼多多PG取消成功
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddRdcPddgeniusSendgoodsCancel {
    
    /// param
    #[serde(rename = "param")]
    pub param: Option<Param>,
    
}


/// 用于未发货仅退款服务商通知拼多多PG取消成功
impl Request for PddRdcPddgeniusSendgoodsCancel {
    fn get_type() -> String {
        "pdd.rdc.pddgenius.sendgoods.cancel".to_string()
    }

    fn get_response_name() -> String {
        "rdc_pddgenius_sendgoods_cancel_response".to_string()
    }
}
