use crate::Request;

use serde::{Deserialize, Serialize};


/// 云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PrintDataList {
    
    /// 自定区打印数据
    #[serde(rename = "custom_area_print_data")]
    pub custom_area_print_data: Option<CustomAreaPrintData>,
    
    /// 面单打印数据
    #[serde(rename = "waybill_printer_data")]
    pub waybill_printer_data: Option<WaybillPrinterData>,
    
}

/// 云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WaybillPrinterData {
    
    /// 追加数据,例如：{\"sender\":{\"address\":{\"province\":\"辽宁\",\"city\":\"沈阳市\",\"district\":\"铁西区\",\"detail\":\"xxx\"},\"name\":\"xxx\",\"mobile\":\"139xxxx032\"}}
    #[serde(rename = "add_data")]
    pub add_data: Option<String>,
    
    /// 打印数据
    #[serde(rename = "data")]
    pub data: Option<String>,
    
    /// 是否加密
    #[serde(rename = "encrypted")]
    pub encrypted: Option<bool>,
    
    /// 签名
    #[serde(rename = "signature")]
    pub signature: Option<String>,
    
    /// 模板url
    #[serde(rename = "template_url")]
    pub template_url: Option<String>,
    
    /// 版本
    #[serde(rename = "ver")]
    pub ver: Option<String>,
    
}

/// 云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddCloudPrint {
    
    /// 云打印请求
    #[serde(rename = "cloud_print_request")]
    pub cloud_print_request: Option<CloudPrintRequest>,
    
}

/// 云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PrinterSetting {
    
    /// 是否打印下联logo
    #[serde(rename = "need_bottom_logo")]
    pub need_bottom_logo: Option<bool>,
    
    /// 是否打印中联logo
    #[serde(rename = "need_middle_logo")]
    pub need_middle_logo: Option<bool>,
    
    /// 是否打印上联logo
    #[serde(rename = "need_top_logo")]
    pub need_top_logo: Option<bool>,
    
    /// 打印方向 normal-正常 reverse-翻转
    #[serde(rename = "orientation")]
    pub orientation: Option<String>,
    
}

/// 云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CloudPrintRequest {
    
    /// 打印数据列表
    #[serde(rename = "print_data_list")]
    pub print_data_list: Option<Vec<PrintDataList>>,
    
    /// 打印机id
    #[serde(rename = "printer_id")]
    pub printer_id: Option<String>,
    
    /// 打印机设置
    #[serde(rename = "printer_setting")]
    pub printer_setting: Option<PrinterSetting>,
    
    /// 共享码
    #[serde(rename = "share_code")]
    pub share_code: Option<String>,
    
}

/// 云打印接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CustomAreaPrintData {
    
    /// 打印数据
    #[serde(rename = "data")]
    pub data: Option<String>,
    
    /// 模板url
    #[serde(rename = "template_url")]
    pub template_url: Option<String>,
    
}


/// 云打印接口
impl Request for PddCloudPrint {
    fn get_type() -> String {
        "pdd.cloud.print".to_string()
    }

    fn get_response_name() -> String {
        "cloud_print_response".to_string()
    }
}
