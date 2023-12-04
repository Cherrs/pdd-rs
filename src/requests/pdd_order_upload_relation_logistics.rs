use crate::Request;

use serde::{Deserialize, Serialize};


/// 针对一笔订单存在多次发货的场景使用，包括但不仅限于补发，换货，分批次发货，线下手工订单等能够匹配到拼多多平台订单号的场景。商家将关联的发货运单号上传后，将用作提升消费者末端取件体验。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExtraTrackList {
    
    /// 快递公司id
    #[serde(rename = "shipping_id")]
    pub shipping_id: Option<i32>,
    
    /// 快递单号
    #[serde(rename = "tracking_number")]
    pub tracking_number: Option<String>,
    
}

/// 针对一笔订单存在多次发货的场景使用，包括但不仅限于补发，换货，分批次发货，线下手工订单等能够匹配到拼多多平台订单号的场景。商家将关联的发货运单号上传后，将用作提升消费者末端取件体验。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOrderUploadRelationLogistics {
    
    /// 订单多包裹发货时使用的其他发货快递信息
    #[serde(rename = "extra_track_list")]
    pub extra_track_list: Option<Vec<ExtraTrackList>>,
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
}


/// 针对一笔订单存在多次发货的场景使用，包括但不仅限于补发，换货，分批次发货，线下手工订单等能够匹配到拼多多平台订单号的场景。商家将关联的发货运单号上传后，将用作提升消费者末端取件体验。
impl Request for PddOrderUploadRelationLogistics {
    fn get_type() -> String {
        "pdd.order.upload.relation.logistics".to_string()
    }

    fn get_response_name() -> String {
        "upload_extra_logistics_response".to_string()
    }
}
