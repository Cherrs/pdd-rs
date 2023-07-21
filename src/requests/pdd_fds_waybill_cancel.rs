use crate::Request;

use serde::{Deserialize, Serialize};


/// 厂家回传完电子面单，需要删除之前上传的电子面单，可以使用该接口取消回传
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFdsWaybillCancelRequest {
    
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

/// 厂家回传完电子面单，需要删除之前上传的电子面单，可以使用该接口取消回传
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFdsWaybillCancel {
    
    /// 入参信息
    #[serde(rename = "pdd_fds_waybill_cancel_request")]
    pub pdd_fds_waybill_cancel_request: Option<PddFdsWaybillCancelRequest>,
    
}


impl Request for PddFdsWaybillCancel {
    fn get_type() -> String {
        "pdd.fds.waybill.cancel".to_string()
    }

    fn get_response_name() -> String {
        "pdd_fds_waybill_cancel_response".to_string()
    }
}
