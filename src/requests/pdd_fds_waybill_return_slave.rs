use crate::Request;

use serde::{Deserialize, Serialize};


/// 1、代发管理支持厂家回传额外运单号给商家2、商家可以在MMS后台的代发管理查看额外运单号字段3、1个订单最多支持上传10个额外运单号，额外的从运单号不能包括主运单号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFdsWaybillReturnSlave {
    
    /// 回传从运单号请求
    #[serde(rename = "request")]
    pub request: Option<PddFdsWaybillReturnSlaveRequest>,
    
}

/// 1、代发管理支持厂家回传额外运单号给商家2、商家可以在MMS后台的代发管理查看额外运单号字段3、1个订单最多支持上传10个额外运单号，额外的从运单号不能包括主运单号
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFdsWaybillReturnSlaveRequest {
    
    /// 代打店铺id
    #[serde(rename = "mall_mask_id")]
    pub mall_mask_id: Option<String>,
    
    /// 代打订单号
    #[serde(rename = "order_mask_sn")]
    pub order_mask_sn: Option<String>,
    
    /// 从运单号列表，最多传递十条从运单号
    #[serde(rename = "waybill_codes")]
    pub waybill_codes: Option<Vec<String>>,
    
    /// 物流公司 Code ，枚举： YTO- 圆通，ZTO-中通，YUNDA-韵达，STO-申通
    #[serde(rename = "wp_code")]
    pub wp_code: Option<String>,
    
}


impl Request for PddFdsWaybillReturnSlave {
    fn get_type() -> String {
        "pdd.fds.waybill.return.slave".to_string()
    }

    fn get_response_name() -> String {
        "pdd_fds_waybill_return_slave_response".to_string()
    }
}
