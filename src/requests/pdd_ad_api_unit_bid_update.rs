use crate::Request;

use serde::{Deserialize, Serialize};


/// 更新单个定向/资源位
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdUnitBids {
    
    /// 出价Id
    #[serde(rename = "bidId")]
    pub bid_id: Option<i64>,
    
    /// 出价，万分比
    #[serde(rename = "bidValue")]
    pub bid_value: Option<i64>,
    
}

/// 更新单个定向/资源位
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitBidUpdate {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 更新列表
    #[serde(rename = "adUnitBids")]
    pub ad_unit_bids: Option<Vec<AdUnitBids>>,
    
    /// 出价资源类型。1表示人群定向，2表示资源位。
    #[serde(rename = "bidReferenceType")]
    pub bid_reference_type: Option<i32>,
    
}


impl Request for PddAdApiUnitBidUpdate {
    fn get_type() -> String {
        "pdd.ad.api.unit.bid.update".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
