use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询已创建的限时限量购活动列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPromotionLimitedDiscountListGet {
    
    /// 支持多个活动类型的查询。3-限量折扣；12-限时折扣。
    #[serde(rename = "activity_types")]
    pub activity_types: Option<Vec<i32>>,
    
    /// 商品id列表
    #[serde(rename = "goods_id_list")]
    pub goods_id_list: Option<Vec<i64>>,
    
    /// 默认false。true-仅返回活动数量；false-返回活动数量和活动设置数据
    #[serde(rename = "just_count")]
    pub just_count: Option<bool>,
    
    /// 0:创建时间升序  1:创建时间降序
    #[serde(rename = "order")]
    pub order: Option<i32>,
    
    /// 页码，默认1
    #[serde(rename = "page_no")]
    pub page_no: Option<i32>,
    
    /// 页大小，默认10
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 支持多个活动状态的查询。1-未开始，2-进行中，3-结束|系统结束，4-结束|商家结束，5-提前结束，6-商品处罚中（屏蔽中）。其中3,4,5都算是结束状态。
    #[serde(rename = "status_list")]
    pub status_list: Option<Vec<i32>>,
    
}


impl Request for PddPromotionLimitedDiscountListGet {
    fn get_type() -> String {
        "pdd.promotion.limited.discount.list.get".to_string()
    }

    fn get_response_name() -> String {
        "online_sum".to_string()
    }
}
