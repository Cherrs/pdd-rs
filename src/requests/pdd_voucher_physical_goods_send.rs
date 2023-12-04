use crate::Request;

use serde::{Deserialize, Serialize};


/// 第三方ISV将商家发货（实物）信息同步给平台
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVoucherPhysicalGoodsSend {
    
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
    
    /// 收件人
    #[serde(rename = "recipient")]
    pub recipient: Option<String>,
    
    /// 收件人电话
    #[serde(rename = "recipient_mobile")]
    pub recipient_mobile: Option<String>,
    
    /// 收件人地址
    #[serde(rename = "recipient_address")]
    pub recipient_address: Option<String>,
    
    /// 物流单号
    #[serde(rename = "logistics_no")]
    pub logistics_no: Option<String>,
    
    /// 物流公司编号
    #[serde(rename = "logistics_company_id")]
    pub logistics_company_id: Option<String>,
    
    /// 物流公司名称
    #[serde(rename = "logistics_company")]
    pub logistics_company: Option<String>,
    
}

/// 第三方ISV将商家发货（实物）信息同步给平台
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VoucherList {
    
    /// 卡券ID
    #[serde(rename = "voucher_id")]
    pub voucher_id: Option<String>,
    
    /// 卡券号
    #[serde(rename = "voucher_no")]
    pub voucher_no: Option<String>,
    
}


/// 第三方ISV将商家发货（实物）信息同步给平台
impl Request for PddVoucherPhysicalGoodsSend {
    fn get_type() -> String {
        "pdd.voucher.physical.goods.send".to_string()
    }

    fn get_response_name() -> String {
        "voucher_physical_voucher_send_response".to_string()
    }
}
