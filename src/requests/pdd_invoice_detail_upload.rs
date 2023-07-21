use crate::Request;

use serde::{Deserialize, Serialize};


/// 第三方ERP在外部开票系统开完发票之后可以调用此接口回传开票结果
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddInvoiceDetailUpload {
    
    /// 申请流水号
    #[serde(rename = "application_id")]
    pub application_id: Option<i64>,
    
    /// 抬头类型：0-个人，1-企业
    #[serde(rename = "business_type")]
    pub business_type: Option<i32>,
    
    /// 开票金额，整数，单位：分
    #[serde(rename = "invoice_amount")]
    pub invoice_amount: Option<i64>,
    
    /// 发票代码
    #[serde(rename = "invoice_code")]
    pub invoice_code: Option<String>,
    
    /// 发票内容，pdf文件(电票回传)，图片文件(专票回传)，转码base64编码
    #[serde(rename = "invoice_file_content")]
    pub invoice_file_content: Option<String>,
    
    /// 多张发票列表（如果本字段为空，invoice_code、invoice_no、invoice_amount、invoice_file_content这四个字段必须填写）
    #[serde(rename = "invoice_item_list")]
    pub invoice_item_list: Option<Vec<InvoiceItemList>>,
    
    /// 发票种类：0-电子发票，1-纸质发票，2-专票；目前只支持0
    #[serde(rename = "invoice_kind")]
    pub invoice_kind: Option<i32>,
    
    /// 发票号码
    #[serde(rename = "invoice_no")]
    pub invoice_no: Option<String>,
    
    /// 开票日期,时间戳（毫秒）
    #[serde(rename = "invoice_time")]
    pub invoice_time: Option<i64>,
    
    /// 开票类型：0-蓝票，1-红票；目前 只支持0
    #[serde(rename = "invoice_type")]
    pub invoice_type: Option<i32>,
    
    /// 备注
    #[serde(rename = "memo")]
    pub memo: Option<String>,
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 原蓝票代码（红票必填）
    #[serde(rename = "original_invoice_code")]
    pub original_invoice_code: Option<String>,
    
    /// 原蓝票号码（红票必填）
    #[serde(rename = "original_invoice_no")]
    pub original_invoice_no: Option<String>,
    
    /// 专票回传必填，专票邮寄快递公司编码，见https://open.pinduoduo.com/application/document/api?id=pdd.logistics.companies.get返回的快递公司编码
    #[serde(rename = "paper_shipping_id")]
    pub paper_shipping_id: Option<i32>,
    
    /// 专票回传必填，专票邮寄运单号
    #[serde(rename = "paper_tracking_number")]
    pub paper_tracking_number: Option<String>,
    
    /// 开票人
    #[serde(rename = "payee_operator")]
    pub payee_operator: Option<String>,
    
    /// （企业抬头）开户账号
    #[serde(rename = "payer_account")]
    pub payer_account: Option<String>,
    
    /// （企业抬头）地址
    #[serde(rename = "payer_address")]
    pub payer_address: Option<String>,
    
    /// （企业抬头）开户银行
    #[serde(rename = "payer_bank")]
    pub payer_bank: Option<String>,
    
    /// 发票抬头
    #[serde(rename = "payer_name")]
    pub payer_name: Option<String>,
    
    /// （企业抬头）电话
    #[serde(rename = "payer_phone")]
    pub payer_phone: Option<String>,
    
    /// 税号，企业必填
    #[serde(rename = "payer_register_no")]
    pub payer_register_no: Option<String>,
    
    /// 不含税金额，整数，单位：分
    #[serde(rename = "sum_price")]
    pub sum_price: Option<i64>,
    
    /// 总税额，整数，单位：分
    #[serde(rename = "sum_tax")]
    pub sum_tax: Option<i32>,
    
    /// 税率,整数
    #[serde(rename = "tax_rate")]
    pub tax_rate: Option<i32>,
    
}

/// 第三方ERP在外部开票系统开完发票之后可以调用此接口回传开票结果
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InvoiceItemList {
    
    /// 开票金额 单位:分
    #[serde(rename = "invoice_amount")]
    pub invoice_amount: Option<i64>,
    
    /// 发票代码
    #[serde(rename = "invoice_code")]
    pub invoice_code: Option<String>,
    
    /// 发票内容，pdf文件(电票回传)，图片文件(专票回传)，转码base64编码
    #[serde(rename = "invoice_file_content")]
    pub invoice_file_content: Option<String>,
    
    /// 发票号码
    #[serde(rename = "invoice_no")]
    pub invoice_no: Option<String>,
    
    /// 原蓝票代码（红票必填）
    #[serde(rename = "original_invoice_code")]
    pub original_invoice_code: Option<String>,
    
    /// 原蓝票号码（红票必填）
    #[serde(rename = "original_invoice_no")]
    pub original_invoice_no: Option<String>,
    
}


impl Request for PddInvoiceDetailUpload {
    fn get_type() -> String {
        "pdd.invoice.detail.upload".to_string()
    }

    fn get_response_name() -> String {
        "invoice_detail_upload_response".to_string()
    }
}
