use crate::Request;

use serde::{Deserialize, Serialize};


/// 寄件实操回告
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfirmSenderInfo {
    
    /// 详细地址
    #[serde(rename = "addrDetail")]
    pub addr_detail: Option<String>,
    
    /// 区名称
    #[serde(rename = "areaName")]
    pub area_name: Option<String>,
    
    /// 市名称
    #[serde(rename = "cityName")]
    pub city_name: Option<String>,
    
    /// 联系人姓名
    #[serde(rename = "contactName")]
    pub contact_name: Option<String>,
    
    /// 联系人手机号
    #[serde(rename = "mobile")]
    pub mobile: Option<String>,
    
    /// 省名称
    #[serde(rename = "provName")]
    pub prov_name: Option<String>,
    
    /// 街道名称
    #[serde(rename = "streetName")]
    pub street_name: Option<String>,
    
    /// 联系人电话号码
    #[serde(rename = "telephone")]
    pub telephone: Option<String>,
    
}

/// 寄件实操回告
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddServiceMailBizactionNotifyRequest {
    
    /// 扩展信息，json格式
    #[serde(rename = "attributes")]
    pub attributes: Option<String>,
    
    /// accept-接单， reject-拒单， dispatch-派单（需要小件员信息）， takePackage-已取件（需要有核重核价信息）， payed-支付完成 postmanCancel-小件员取消， expressCancel-快递公司取消 postMailNo-回传运单
    #[serde(rename = "bizAction")]
    pub biz_action: Option<String>,
    
    /// 服务明细描述信息
    #[serde(rename = "bizActionDesc")]
    pub biz_action_desc: Option<String>,
    
    /// 取消原因
    #[serde(rename = "cancelOrRejectReason")]
    pub cancel_or_reject_reason: Option<String>,
    
    /// 核价金额，单位分
    #[serde(rename = "checkPrice")]
    pub check_price: Option<i32>,
    
    /// 核价方式（当前实操为checkWeightPrice时必填）： weight:按照重量核价； volume：按照体积核价
    #[serde(rename = "checkPriceType")]
    pub check_price_type: Option<String>,
    
    /// 核重重量，单位克
    #[serde(rename = "checkWeight")]
    pub check_weight: Option<i32>,
    
    /// 最终服务商确认的收件人信息
    #[serde(rename = "confirmReceiverInfo")]
    pub confirm_receiver_info: Option<ConfirmReceiverInfo>,
    
    /// 最终服务商确认的寄件人信息
    #[serde(rename = "confirmSenderInfo")]
    pub confirm_sender_info: Option<ConfirmSenderInfo>,
    
    /// 改约结束时间- changeAppoint必填
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    
    /// 业务实操时间
    #[serde(rename = "executeTime")]
    pub execute_time: Option<String>,
    
    /// 柜子编号
    #[serde(rename = "expressBoxCode")]
    pub express_box_code: Option<String>,
    
    /// 物流公司编码
    #[serde(rename = "expressCode")]
    pub express_code: Option<String>,
    
    /// 运费金额
    #[serde(rename = "freightPrice")]
    pub freight_price: Option<i32>,
    
    /// 高度（当前核价方式为volume时必填），单位是厘米
    #[serde(rename = "height")]
    pub height: Option<i32>,
    
    /// 保费，单位分
    #[serde(rename = "insurancePrice")]
    pub insurance_price: Option<i32>,
    
    /// 报价金额，单位分
    #[serde(rename = "insuranceValue")]
    pub insurance_value: Option<i32>,
    
    /// 长度（当前核价方式为volume时必填），单位是厘米
    #[serde(rename = "length")]
    pub length: Option<i32>,
    
    /// 运单号
    #[serde(rename = "mailNo")]
    pub mail_no: Option<String>,
    
    /// 寄件订单单号
    #[serde(rename = "mailOrderSn")]
    pub mail_order_sn: Option<String>,
    
    /// 小件员修改后的收件人信息
    #[serde(rename = "modifyReceiverInfo")]
    pub modify_receiver_info: Option<ModifyReceiverInfo>,
    
    /// 其他费用，单位分
    #[serde(rename = "otherPrice")]
    pub other_price: Option<i32>,
    
    /// 包装费用，单位分
    #[serde(rename = "packagePrice")]
    pub package_price: Option<i32>,
    
    /// 支付金额，单位分
    #[serde(rename = "payPrice")]
    pub pay_price: Option<i32>,
    
    /// 取件码
    #[serde(rename = "pickCode")]
    pub pick_code: Option<String>,
    
    /// 寄件类型，HOME_DELIVERY-上门取件
    #[serde(rename = "postType")]
    pub post_type: Option<String>,
    
    /// 小件员code
    #[serde(rename = "postmanCode")]
    pub postman_code: Option<String>,
    
    /// 小件员姓名
    #[serde(rename = "postmanName")]
    pub postman_name: Option<String>,
    
    /// 小件员电话
    #[serde(rename = "postmanPhone")]
    pub postman_phone: Option<String>,
    
    /// 取消原因code
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<String>,
    
    /// 滞留原因
    #[serde(rename = "retentionReason")]
    pub retention_reason: Option<String>,
    
    /// 改约开始时间- changeAppoint节点必填
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    
    /// 宽度（当前核价方式为volume时必填），单位是厘米
    #[serde(rename = "width")]
    pub width: Option<i32>,
    
}

/// 寄件实操回告
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ModifyReceiverInfo {
    
    /// 详细地址
    #[serde(rename = "addrDetail")]
    pub addr_detail: Option<String>,
    
    /// 区名称
    #[serde(rename = "areaName")]
    pub area_name: Option<String>,
    
    /// 市名称
    #[serde(rename = "cityName")]
    pub city_name: Option<String>,
    
    /// 联系人姓名
    #[serde(rename = "contactName")]
    pub contact_name: Option<String>,
    
    /// 联系人手机号
    #[serde(rename = "mobile")]
    pub mobile: Option<String>,
    
    /// 省名称
    #[serde(rename = "provName")]
    pub prov_name: Option<String>,
    
    /// 街道名称
    #[serde(rename = "streetName")]
    pub street_name: Option<String>,
    
    /// 联系人电话号码
    #[serde(rename = "telephone")]
    pub telephone: Option<String>,
    
}

/// 寄件实操回告
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfirmReceiverInfo {
    
    /// 详细地址
    #[serde(rename = "addrDetail")]
    pub addr_detail: Option<String>,
    
    /// 区名称
    #[serde(rename = "areaName")]
    pub area_name: Option<String>,
    
    /// 市名称
    #[serde(rename = "cityName")]
    pub city_name: Option<String>,
    
    /// 联系人姓名
    #[serde(rename = "contactName")]
    pub contact_name: Option<String>,
    
    /// 联系人手机号
    #[serde(rename = "mobile")]
    pub mobile: Option<String>,
    
    /// 省名称
    #[serde(rename = "provName")]
    pub prov_name: Option<String>,
    
    /// 街道名称
    #[serde(rename = "streetName")]
    pub street_name: Option<String>,
    
    /// 联系人电话号码
    #[serde(rename = "telephone")]
    pub telephone: Option<String>,
    
}

/// 寄件实操回告
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddServiceMailBizactionNotify {
    
    /// 请求参数
    #[serde(rename = "request")]
    pub request: Option<PddServiceMailBizactionNotifyRequest>,
    
}


impl Request for PddServiceMailBizactionNotify {
    fn get_type() -> String {
        "pdd.service.mail.bizaction.notify".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
