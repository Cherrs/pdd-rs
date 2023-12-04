use crate::Request;

use serde::{Deserialize, Serialize};


/// 售后列表增量查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddRefundListIncrementGet {
    
    /// 必填，售后状态 0：无售后 2：买家申请退款，待商家处理 3：退货退款，待商家处理 4：商家同意退款，退款中 5：平台同意退款，退款中 6：驳回退款，待买家处理 7：已同意退货退款,待用户发货 8：平台处理中 9：平台拒绝退款，退款关闭 10：退款成功 11：买家撤销 12：买家逾期未处理，退款失败 13：买家逾期，超过有效期 14：换货补寄待商家处理 15：换货补寄待用户处理 16：换货补寄成功 17：换货补寄失败 18：换货补寄待用户确认完成 21：待商家同意维修 22：待用户确认发货 24：维修关闭 25：维修成功 27：待用户确认收货 31：已同意拒收退款，待用户拒收 32：补寄待商家发货 33：待商家召回
    #[serde(rename = "after_sales_status")]
    pub after_sales_status: Option<i32>,
    
    /// 必填，售后类型 1：全部 2：仅退款 3：退货退款 4：换货 5：缺货补寄 6：维修
    #[serde(rename = "after_sales_type")]
    pub after_sales_type: Option<i32>,
    
    /// 必填，最后更新时间结束时间的UNIX时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时00 分 00 秒)起至现在的总秒数 PS：开始时间结束时间间距不超过 30 分钟
    #[serde(rename = "end_updated_at")]
    pub end_updated_at: Option<i64>,
    
    /// 订单号。若入参含订单号，则可查询订单下的全部售后单。且入参中除订单号，page，page_size外的其他查询条件不起作用（标记必填的仍旧需要输入）。
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
    /// 返回页码 默认 1，页码从 1 开始 PS：当前采用分页返回，数量和页数会一起传，如果不传，则采用 默认值
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 返回数量，默认 100。最大 100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 必填，最后更新时间开始时间的UNIX时间戳，指格林威治时间 1970 年01月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00分 00 秒)起至现在的总秒数
    #[serde(rename = "start_updated_at")]
    pub start_updated_at: Option<i64>,
    
}


/// 售后列表增量查询
impl Request for PddRefundListIncrementGet {
    fn get_type() -> String {
        "pdd.refund.list.increment.get".to_string()
    }

    fn get_response_name() -> String {
        "refund_increment_get_response".to_string()
    }
}
