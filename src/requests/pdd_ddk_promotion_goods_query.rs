use crate::Request;

use serde::{Deserialize, Serialize};


/// 信息流渠道进行商品投放报备后，渠道可使用该接口进行报备商品审批进度查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkPromotionGoodsQuery {
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 店铺id
    #[serde(rename = "mall_id")]
    pub mall_id: Option<i64>,
    
    /// 分页查询页数
    #[serde(rename = "page_number")]
    pub page_number: Option<i32>,
    
    /// 分页查询页大小
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 查询状态列表
    #[serde(rename = "status_list")]
    pub status_list: Option<Vec<i32>>,
    
    /// 最后更新开始时间
    #[serde(rename = "update_end_time")]
    pub update_end_time: Option<i64>,
    
    /// 最后更新结束时间（最长支持30天）
    #[serde(rename = "update_start_time")]
    pub update_start_time: Option<i64>,
    
}


/// 信息流渠道进行商品投放报备后，渠道可使用该接口进行报备商品审批进度查询
impl Request for PddDdkPromotionGoodsQuery {
    fn get_type() -> String {
        "pdd.ddk.promotion.goods.query".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
