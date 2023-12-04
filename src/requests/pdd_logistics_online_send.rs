use crate::Request;

use serde::{Deserialize, Serialize};


/// 订单发货通知
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddLogisticsOnlineSend {
    
    /// 发货个性内容，支持imei（手机串号），deviceSn（设备序列号），overseaTracing（海淘溯源码id）内容，appraisalCert（商品证书编号）。形如：“imei=识别码1,识别码2;”、“ deviceSn=序列号1,序列号2;”、“ organicCode=有机码1,有机码2;”、“overseaTracing=溯源码1,溯源码2;”、“appraisalCert=商品证书编号1;”。以英文逗号“,”分割串号，以英文分号“;”分割不同参数内容。上传时请严格区分imei，deviceSn，organicCode，overseaTracing和appraisalCert，其中overseaTracing（海淘溯源码id）要求海淘商品在支持溯源的情况下必传，appraisalCert（商品证书编号）要求珠宝类商品在支持专业鉴定的情况下必传；以上错传/漏传将会导致发货失败
    #[serde(rename = "feature")]
    pub feature: Option<String>,
    
    /// 快递公司编号
    #[serde(rename = "logistics_id")]
    pub logistics_id: Option<i64>,
    
    /// 订单号。形如：20150909-452750051
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 修改发货模式：不传则默认为首次发货 1=首次发货：用于订单首次发货，仅待发货订单可传入； 2=修改发货：用于订单修改发货，调用成功后将会覆盖原发货信息，仅已发货订单可传入
    #[serde(rename = "redelivery_type")]
    pub redelivery_type: Option<i32>,
    
    /// 退货地址的id，不填则取商品默认退货地址（可在“拼多多-商家后台/售后工作台/售后设置”为商品绑定默认退货地址，若未设置则取店铺默认退货地址）
    #[serde(rename = "refund_address_id")]
    pub refund_address_id: Option<String>,
    
    /// 快递单号
    #[serde(rename = "tracking_number")]
    pub tracking_number: Option<String>,
    
}


/// 订单发货通知
impl Request for PddLogisticsOnlineSend {
    fn get_type() -> String {
        "pdd.logistics.online.send".to_string()
    }

    fn get_response_name() -> String {
        "logistics_online_send_response".to_string()
    }
}
