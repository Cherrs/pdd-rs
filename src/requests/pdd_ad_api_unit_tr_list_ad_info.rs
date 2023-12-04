use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询全站推广广告信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitTrListAdInfo {
    
    /// 报表结束日期 格式：yyyy-MM-dd HH:mm:ss
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    
    /// 商品id列表
    #[serde(rename = "goodsIds")]
    pub goods_ids: Option<Vec<i64>>,
    
    /// 排序字段，支持报表字段枚举：0-曝光，1-点击，2-点击率，3-cpc,4-花费，5-订单量,6-gmv，7-roi,8-日期，9-cpm,10-店铺收藏，11-商品收藏
    #[serde(rename = "orderBy")]
    pub order_by: Option<i32>,
    
    /// 排序类型,0-倒序，1-正序
    #[serde(rename = "sortBy")]
    pub sort_by: Option<i32>,
    
    /// 报表开始日期 格式：yyyy-MM-dd HH:mm:ss
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    
    /// 分页大小 默认10
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    
    /// 分页页数 默认1
    #[serde(rename = "pageNumber")]
    pub page_number: Option<i32>,
    
}


/// 查询全站推广广告信息
impl Request for PddAdApiUnitTrListAdInfo {
    fn get_type() -> String {
        "pdd.ad.api.unit.tr.list.ad.info".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
