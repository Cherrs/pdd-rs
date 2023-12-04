use crate::Request;

use serde::{Deserialize, Serialize};


/// 针对一笔订单分多笔物流发货的场景（分包发货、补寄、发放赠品），将支持商家额外上传运单号，额外运单作为补充信息仅用作消费者查看。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOrderUploadExtraLogistics {
    
    /// 订单多包裹发货时使用的其他发货快递信息
    #[serde(rename = "extra_track_list")]
    pub extra_track_list: Option<Vec<ExtraTrackList>>,
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 额外运单类型，1=分包发货，2=补发商品，3=发放赠品
    #[serde(rename = "extra_track_type")]
    pub extra_track_type: Option<i32>,
    
}

/// 针对一笔订单分多笔物流发货的场景（分包发货、补寄、发放赠品），将支持商家额外上传运单号，额外运单作为补充信息仅用作消费者查看。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExtraTrackList {
    
    /// 快递公司id
    #[serde(rename = "shipping_id")]
    pub shipping_id: Option<i32>,
    
    /// 快递单号
    #[serde(rename = "tracking_number")]
    pub tracking_number: Option<String>,
    
}


/// 针对一笔订单分多笔物流发货的场景（分包发货、补寄、发放赠品），将支持商家额外上传运单号，额外运单作为补充信息仅用作消费者查看。
impl Request for PddOrderUploadExtraLogistics {
    fn get_type() -> String {
        "pdd.order.upload.extra.logistics".to_string()
    }

    fn get_response_name() -> String {
        "upload_extra_logistics_response".to_string()
    }
}
