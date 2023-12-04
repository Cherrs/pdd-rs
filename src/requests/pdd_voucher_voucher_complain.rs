use crate::Request;

use serde::{Deserialize, Serialize};


/// 卡券投诉接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVoucherVoucherComplain {
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 外部流水号
    #[serde(rename = "out_biz_no")]
    pub out_biz_no: Option<String>,
    
    /// 优惠券信息列表,例子[{"voucher_id":"test voucher_id","voucher_no":"test voucher_no"}]
    #[serde(rename = "voucher_list")]
    pub voucher_list: Option<Vec<VoucherList>>,
    
    /// 投诉人
    #[serde(rename = "complain_user")]
    pub complain_user: Option<String>,
    
    /// 投诉人电话
    #[serde(rename = "complain_user_mobile")]
    pub complain_user_mobile: Option<String>,
    
    /// 投诉内容
    #[serde(rename = "complain_content")]
    pub complain_content: Option<String>,
    
    /// ["http://testimg.yangkeduo.com/pdd_oms/2018-01-16/411068e948835ae053a86c13f8ebb5ee.jpg"]
    #[serde(rename = "complain_attachment_list")]
    pub complain_attachment_list: Option<Vec<String>>,
    
    /// 枚举值1、大闸蟹死蟹或者少蟹 ；2、大闸蟹重量不符；3、大闸蟹公母数量不符；4、大闸蟹产地不符；5、欺诈发货（收到的产品非大闸蟹）；6、蟹券无法提货7、其他质量问题
    #[serde(rename = "complain_type")]
    pub complain_type: Option<i32>,
    
}

/// 卡券投诉接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VoucherList {
    
    /// 卡券ID
    #[serde(rename = "voucher_id")]
    pub voucher_id: Option<String>,
    
    /// 卡券号
    #[serde(rename = "voucher_no")]
    pub voucher_no: Option<String>,
    
}


/// 卡券投诉接口
impl Request for PddVoucherVoucherComplain {
    fn get_type() -> String {
        "pdd.voucher.voucher.complain".to_string()
    }

    fn get_response_name() -> String {
        "voucher_voucher_complain_response".to_string()
    }
}
