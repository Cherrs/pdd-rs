use crate::Request;

use serde::{Deserialize, Serialize};


/// 厂家回传电子面单到商家订单
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFdsWaybillReturn {
    
    /// 入参信息
    #[serde(rename = "param_fds_waybill_return_request")]
    pub param_fds_waybill_return_request: Option<ParamFdsWaybillReturnRequest>,
    
}

/// 厂家回传电子面单到商家订单
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParamFdsWaybillReturnRequest {
    
    /// 代打店铺id
    #[serde(rename = "mall_mask_id")]
    pub mall_mask_id: Option<String>,
    
    /// 代打订单号
    #[serde(rename = "order_mask_sn")]
    pub order_mask_sn: Option<String>,
    
    /// 面单号
    #[serde(rename = "waybill_code")]
    pub waybill_code: Option<String>,
    
    /// 物流公司 Code ，枚举： YTO- 圆通，ZTO-中通，YUNDA-韵达，STO-申通
    #[serde(rename = "wp_code")]
    pub wp_code: Option<String>,
    
}


impl Request for PddFdsWaybillReturn {
    fn get_type() -> String {
        "pdd.fds.waybill.return".to_string()
    }

    fn get_response_name() -> String {
        "pdd_fds_waybill_return_response".to_string()
    }
}
