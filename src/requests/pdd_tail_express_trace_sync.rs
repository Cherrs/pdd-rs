use crate::Request;

use serde::{Deserialize, Serialize};


/// 集运末端业务，物流商回传未拼接二段物流的三段轨迹信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTailExpressTraceSync {
    
    /// 轨迹信息
    #[serde(rename = "request")]
    pub request: Option<PddTailExpressTraceSyncRequest>,
    
}

/// 集运末端业务，物流商回传未拼接二段物流的三段轨迹信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTailExpressTraceSyncRequest {
    
    /// 地址
    #[serde(rename = "address")]
    pub address: Option<String>,
    
    /// 扫描城市名称
    #[serde(rename = "city")]
    pub city: Option<String>,
    
    /// 轨迹详情描述
    #[serde(rename = "description")]
    pub description: Option<String>,
    
    /// 三级地址，区/县
    #[serde(rename = "district")]
    pub district: Option<String>,
    
    /// 问题件原因code
    #[serde(rename = "failReason")]
    pub fail_reason: Option<String>,
    
    /// 数据id java.util.UUID生成
    #[serde(rename = "id")]
    pub id: Option<String>,
    
    /// 操作时间 格式：yyyy-MM-dd hh:mm:ss
    #[serde(rename = "operationTime")]
    pub operation_time: Option<String>,
    
    /// 省份
    #[serde(rename = "province")]
    pub province: Option<String>,
    
    /// 快递公司id
    #[serde(rename = "shippingId")]
    pub shipping_id: Option<i64>,
    
    /// 扫描站点名称
    #[serde(rename = "siteName")]
    pub site_name: Option<String>,
    
    /// 扫描站点编码 站点编号(各快递公司用于区分站点的唯一id)
    #[serde(rename = "siteNo")]
    pub site_no: Option<String>,
    
    /// 扫描站点类型 1:网点；2:中转中心；3:代收点
    #[serde(rename = "siteType")]
    pub site_type: Option<i32>,
    
    /// 轨迹状态 如：GOT、SEND
    #[serde(rename = "status")]
    pub status: Option<String>,
    
    /// 运单号
    #[serde(rename = "trackingNumber")]
    pub tracking_number: Option<String>,
    
    /// 物流号 物流订单号
    #[serde(rename = "trackingOrderNo")]
    pub tracking_order_no: Option<String>,
    
}


/// 集运末端业务，物流商回传未拼接二段物流的三段轨迹信息
impl Request for PddTailExpressTraceSync {
    fn get_type() -> String {
        "pdd.tail.express.trace.sync".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
