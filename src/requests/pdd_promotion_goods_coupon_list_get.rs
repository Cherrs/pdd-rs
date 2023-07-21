use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品优惠券批次列表查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddPromotionGoodsCouponListGet {
    
    /// 页码，默认1
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 每页数量，默认100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 商品ID
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 查询范围0 全部，1 多多进宝券，2 无门槛商品券；默认1
    #[serde(rename = "query_range")]
    pub query_range: Option<i32>,
    
    /// 批次状态1 领取中，2 已领完，3 已结束，4 已暂停
    #[serde(rename = "batch_status")]
    pub batch_status: Option<i32>,
    
    /// 排序1 创建时间正序，2 创建时间倒序，3 开始时间正序，4 开始时间倒序，5 初始数量正序， 6 初始数量倒序，7 领取数量正序，8 领取数量倒序；默认2
    #[serde(rename = "sort_by")]
    pub sort_by: Option<i32>,
    
}


impl Request for PddPromotionGoodsCouponListGet {
    fn get_type() -> String {
        "pdd.promotion.goods.coupon.list.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_coupon_batch_list_response".to_string()
    }
}
