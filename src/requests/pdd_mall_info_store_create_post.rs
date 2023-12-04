use crate::Request;

use serde::{Deserialize, Serialize};


/// 创建店铺门店
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddMallInfoStoreCreatePost {
    
    /// 门店营业状态
    #[serde(rename = "business_status")]
    pub business_status: Option<i32>,
    
    /// 营业天数
    #[serde(rename = "business_week_list")]
    pub business_week_list: Option<Vec<i32>>,
    
    /// 省市区三级地址-市名
    #[serde(rename = "city")]
    pub city: Option<String>,
    
    /// 省市区三级地址-区名
    #[serde(rename = "district")]
    pub district: Option<String>,
    
    /// 结束营业时间段
    #[serde(rename = "end_business_hour")]
    pub end_business_hour: Option<String>,
    
    /// 腾讯地图POI信息ID
    #[serde(rename = "poi_id")]
    pub poi_id: Option<String>,
    
    /// 门店纬度
    #[serde(rename = "poi_latitude")]
    pub poi_latitude: Option<f32>,
    
    /// 门店经度
    #[serde(rename = "poi_longitude")]
    pub poi_longitude: Option<f32>,
    
    /// 省市区三级地址-省名
    #[serde(rename = "province")]
    pub province: Option<String>,
    
    /// 起始营业时间段
    #[serde(rename = "start_business_hour")]
    pub start_business_hour: Option<String>,
    
    /// 门店名称
    #[serde(rename = "store_name")]
    pub store_name: Option<String>,
    
    /// 门店自有编号
    #[serde(rename = "store_number")]
    pub store_number: Option<String>,
    
    /// 门店电话
    #[serde(rename = "store_phone")]
    pub store_phone: Option<String>,
    
    /// 门店行业（1-男女装，2-运动户外，3-服饰配件，4-厨具电器，5-汽车，8-全屋定制）
    #[serde(rename = "trade_type")]
    pub trade_type: Option<i32>,
    
}


/// 创建店铺门店
impl Request for PddMallInfoStoreCreatePost {
    fn get_type() -> String {
        "pdd.mall.info.store.create.post".to_string()
    }

    fn get_response_name() -> String {
        "mall_info_store_create_post_response".to_string()
    }
}
