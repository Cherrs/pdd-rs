use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询创意列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitCreativeQueryList {
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
    /// 报表数据开始日期
    #[serde(rename = "beginDate")]
    pub begin_date: Option<String>,
    
    /// 报表数据截止日期
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    
    /// 排序字段。0表示按曝光量排序，1表示按点击量排序，2表示按点击率排序，3表示按点击单价排序，4表示按消耗排序，5表示按订单数排序，6表示按交易额排序，7表示按产出比排序，8表示按日期排序，9表示按千次曝光单价排序，10表示按店铺收藏数排序，11表示按商品收藏数排序，12表示按点击转化率排序，13表示按转化成本排序，14表示按平均成交金额排序。
    #[serde(rename = "orderBy")]
    pub order_by: Option<i32>,
    
    /// 排序类型。0表示降序，1表示升序。
    #[serde(rename = "sortBy")]
    pub sort_by: Option<i32>,
    
}


impl Request for PddAdApiUnitCreativeQueryList {
    fn get_type() -> String {
        "pdd.ad.api.unit.creative.query.list".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
