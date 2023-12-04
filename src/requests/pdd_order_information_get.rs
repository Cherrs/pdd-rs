
use crate::Request;
use serde::{Deserialize, Serialize};

/// 查询单个订单详情（只能获取到成交时间三个月以内的交易信息）注：虚拟订单充值手机号信息无法通过此接口获取，请联系虚拟类目运营人员。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOrderInformationGet {
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
}

/// 查询单个订单详情（只能获取到成交时间三个月以内的交易信息）
/// 注：虚拟订单充值手机号信息无法通过此接口获取，请联系虚拟类目运营人员。
impl Request for PddOrderInformationGet {
    fn get_type() -> String {
        "pdd.order.information.get".to_string()
    }

    fn get_response_name() -> String {
        "order_info_get_response".to_string()
    }
}
