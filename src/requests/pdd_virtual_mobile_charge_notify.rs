use crate::Request;

use serde::{Deserialize, Serialize};


/// 虚拟类目发货的接口【仅供话费/流量直冲商家自研对接进行话费流量发货使用】
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVirtualMobileChargeNotify {
    
    /// 直充附加信息对象数组
    #[serde(rename = "charge_certi")]
    pub charge_certi: Option<Vec<ChargeCerti>>,
    
    /// 电子发票信息 0-不支持开具  1-支持开具
    #[serde(rename = "ele_invoice")]
    pub ele_invoice: Option<i32>,
    
    /// 拼多多订单编码
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 11122dafa 外部系统订单编码
    #[serde(rename = "outer_order_sn")]
    pub outer_order_sn: Option<String>,
    
    /// 虚拟系统充值结果，SUCCESS-充值成功，FAIL-充值失败
    #[serde(rename = "status")]
    pub status: Option<String>,
    
}

/// 虚拟类目发货的接口【仅供话费/流量直冲商家自研对接进行话费流量发货使用】
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChargeCerti {
    
    /// 直充充值金额（单位：分）
    #[serde(rename = "charge_certi_amount")]
    pub charge_certi_amount: Option<i64>,
    
    /// 直充充值成功时间 （yyyy-MM-dd HH:mm:ss格式）
    #[serde(rename = "charge_certi_date")]
    pub charge_certi_date: Option<String>,
    
    /// 直充充值号码
    #[serde(rename = "charge_certi_mobile")]
    pub charge_certi_mobile: Option<String>,
    
    /// 充值卡号尾号
    #[serde(rename = "charge_certi_mobile_tail")]
    pub charge_certi_mobile_tail: Option<String>,
    
    /// 直充充值单号
    #[serde(rename = "charge_certi_order_sn")]
    pub charge_certi_order_sn: Option<String>,
    
    /// 直充短信原文
    #[serde(rename = "charge_certi_text")]
    pub charge_certi_text: Option<String>,
    
    /// 代理商(渠道)编号
    #[serde(rename = "merchant_outer_id")]
    pub merchant_outer_id: Option<String>,
    
}


impl Request for PddVirtualMobileChargeNotify {
    fn get_type() -> String {
        "pdd.virtual.mobile.charge.notify".to_string()
    }

    fn get_response_name() -> String {
        "mobile_charge_notify_response".to_string()
    }
}
