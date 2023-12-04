use crate::Request;

use serde::{Deserialize, Serialize};


/// 创建限时限量购活动（包括限时折扣和限量折扣）
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SkuPriceList {
    
    /// sku活动价格
    #[serde(rename = "activity_price")]
    pub activity_price: Option<i64>,
    
    /// skuid
    #[serde(rename = "sku_id")]
    pub sku_id: Option<i64>,
    
}

/// 创建限时限量购活动（包括限时折扣和限量折扣）
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPromotionLimitedActivityCreate {
    
    /// 创建请求
    #[serde(rename = "request")]
    pub request: Option<Vec<PddPromotionLimitedActivityCreateRequest>>,
    
}

/// 创建限时限量购活动（包括限时折扣和限量折扣）
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPromotionLimitedActivityCreateRequest {
    
    /// 活动名称
    #[serde(rename = "activity_name")]
    pub activity_name: Option<String>,
    
    /// 活动类型
    #[serde(rename = "activity_type")]
    pub activity_type: Option<i32>,
    
    /// 折扣比例，实际折扣为：discount/1000。例：300，表示3折
    #[serde(rename = "discount")]
    pub discount: Option<i64>,
    
    /// 限时折扣必填。结束时间（单位：s）
    #[serde(rename = "end_time")]
    pub end_time: Option<i64>,
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 活动库存的数量（限量活动时，必填）
    #[serde(rename = "quantity")]
    pub quantity: Option<i64>,
    
    /// 参与活动的sku数据列表
    #[serde(rename = "sku_price_list")]
    pub sku_price_list: Option<Vec<SkuPriceList>>,
    
    /// 限时折扣必填。开始时间（单位：s）
    #[serde(rename = "start_time")]
    pub start_time: Option<i64>,
    
    /// 用户限购数量(0:不限购)
    #[serde(rename = "user_activity_limit")]
    pub user_activity_limit: Option<i64>,
    
}


/// 创建限时限量购活动（包括限时折扣和限量折扣）
impl Request for PddPromotionLimitedActivityCreate {
    fn get_type() -> String {
        "pdd.promotion.limited.activity.create".to_string()
    }

    fn get_response_name() -> String {
        "result_list".to_string()
    }
}
