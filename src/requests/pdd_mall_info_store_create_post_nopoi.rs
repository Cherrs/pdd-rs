use crate::Request;

use serde::{Deserialize, Serialize};


/// 开放平台无PoiId创建门店
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoStoreCreatePostNopoi {
    
    /// 门店营业状态（0:营业中 1:暂停营业）
    #[serde(rename = "business_status")]
    pub business_status: Option<i32>,
    
    /// 一周营业时间，例：[1,2,3,4,5,6,7]
    #[serde(rename = "business_week_list")]
    pub business_week_list: Option<Vec<i32>>,
    
    /// 市
    #[serde(rename = "city")]
    pub city: Option<String>,
    
    /// 区
    #[serde(rename = "district")]
    pub district: Option<String>,
    
    /// 结束营业时间，例："19:00"
    #[serde(rename = "end_business_hour")]
    pub end_business_hour: Option<String>,
    
    /// 门店纬度
    #[serde(rename = "poi_latitude")]
    pub poi_latitude: Option<f32>,
    
    /// 门店经度
    #[serde(rename = "poi_longitude")]
    pub poi_longitude: Option<f32>,
    
    /// 省
    #[serde(rename = "province")]
    pub province: Option<String>,
    
    /// 起始营业时间，例："07:00"
    #[serde(rename = "start_business_hour")]
    pub start_business_hour: Option<String>,
    
    /// 详细地址
    #[serde(rename = "store_address")]
    pub store_address: Option<String>,
    
    /// 门店名称
    #[serde(rename = "store_name")]
    pub store_name: Option<String>,
    
    /// 门店自有编号
    #[serde(rename = "store_number")]
    pub store_number: Option<String>,
    
    /// 门店电话
    #[serde(rename = "store_phone")]
    pub store_phone: Option<String>,
    
    /// 门店行业类型（1-男女装，2-运动户外，3-服饰配件，4-厨具电器，5-汽车，8-全屋定制）
    #[serde(rename = "trade_type")]
    pub trade_type: Option<i32>,
    
}


/// 开放平台无PoiId创建门店
impl Request for PddMallInfoStoreCreatePostNopoi {
    fn get_type() -> String {
        "pdd.mall.info.store.create.post.nopoi".to_string()
    }

    fn get_response_name() -> String {
        "res".to_string()
    }
}
