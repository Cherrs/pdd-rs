use crate::Request;

use serde::{Deserialize, Serialize};


/// 同步定向/资源位
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AreaStruct {
    
    /// 地域Id列表。具体地域Id编码参见接口返回：pdd.ad.api.unit.bid.query.targeting.tag.list
    #[serde(rename = "areaIds")]
    pub area_ids: Option<Vec<i32>>,
    
}

/// 同步定向/资源位
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdTargetingSet {
    
    /// 地域定向
    #[serde(rename = "areaStruct")]
    pub area_struct: Option<AreaStruct>,
    
}

/// 同步定向/资源位
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdUnitBids {
    
    /// 定向信息。仅支持地域定向。
    #[serde(rename = "adTargetingVO")]
    pub ad_targeting_vo: Option<AdTargetingVo>,
    
    /// 可选人群定向类型或者可选资源位定向类型。人群定向类型，可用枚举值，参考接口：pdd.ad.api.unit.bid.query.base.target.profile资源位定向类型，可用枚举值，参考接口：pdd.ad.api.unit.bid.query.available.location
    #[serde(rename = "bidReferenceId")]
    pub bid_reference_id: Option<i64>,
    
    /// 出价，万分比，10000表示100%
    #[serde(rename = "bidValue")]
    pub bid_value: Option<i64>,
    
    /// 二级定向Id。默认为0。
    #[serde(rename = "subBidReferenceId")]
    pub sub_bid_reference_id: Option<i64>,
    
}

/// 同步定向/资源位
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdTargetingVo {
    
    /// 定向集合
    #[serde(rename = "adTargetingSet")]
    pub ad_targeting_set: Option<AdTargetingSet>,
    
    /// 定向名称
    #[serde(rename = "targetingName")]
    pub targeting_name: Option<String>,
    
}

/// 同步定向/资源位
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitBidSync {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 出价信息列表
    #[serde(rename = "adUnitBids")]
    pub ad_unit_bids: Option<Vec<AdUnitBids>>,
    
    /// 出价资源类型。1表示人群定向，2表示资源位。
    #[serde(rename = "bidReferenceType")]
    pub bid_reference_type: Option<i32>,
    
}


/// 同步定向/资源位
impl Request for PddAdApiUnitBidSync {
    fn get_type() -> String {
        "pdd.ad.api.unit.bid.sync".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
