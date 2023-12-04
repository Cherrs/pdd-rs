use crate::Request;

use serde::{Deserialize, Serialize};


/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TravelerInfoLimitation {
    
    /// 游玩人证件
    #[serde(rename = "credential")]
    pub credential: Option<i32>,
    
    /// 游玩人名字
    #[serde(rename = "name")]
    pub name: Option<i32>,
    
    /// 出游人信息设置
    #[serde(rename = "traveler_required")]
    pub traveler_required: Option<i32>,
    
}

/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TicketTime {
    
    /// 备注
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    
    /// 换票结束时间
    #[serde(rename = "end_at")]
    pub end_at: Option<String>,
    
    /// 换票开始时间
    #[serde(rename = "start_at")]
    pub start_at: Option<String>,
    
}

/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OrderLimitation {
    
    /// 周期长度
    #[serde(rename = "cycle_length")]
    pub cycle_length: Option<i32>,
    
    /// 限制类型
    #[serde(rename = "limitation_type")]
    pub limitation_type: Option<i32>,
    
    /// 周期类型
    #[serde(rename = "limit_cycle")]
    pub limit_cycle: Option<i32>,
    
    /// 限购数量
    #[serde(rename = "limit_num")]
    pub limit_num: Option<i32>,
    
}

/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProviderContactInfo {
    
    /// 服务时间
    #[serde(rename = "provider_business_hour")]
    pub provider_business_hour: Option<Vec<ProviderBusinessHour>>,
    
    /// 服务商名称
    #[serde(rename = "provider_name")]
    pub provider_name: Option<String>,
    
    /// 服务商联系电话
    #[serde(rename = "provider_telephone")]
    pub provider_telephone: Option<String>,
    
}

/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RefundLimitations {
    
    /// 是否可退
    #[serde(rename = "is_refundable")]
    pub is_refundable: Option<i32>,
    
    /// 退款规则
    #[serde(rename = "refund_rules")]
    pub refund_rules: Option<Vec<RefundRules>>,
    
}

/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RefundRules {
    
    /// 游玩日 0 点提前 或之后分钟数
    #[serde(rename = "ahead_time")]
    pub ahead_time: Option<i32>,
    
    /// 扣费值
    #[serde(rename = "deduction_fee")]
    pub deduction_fee: Option<i32>,
    
    /// 费率单位
    #[serde(rename = "deduction_unit")]
    pub deduction_unit: Option<i32>,
    
    /// 规则类型
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    
}

/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BookerInfoLimitation {
    
    /// 需要下单人信息
    #[serde(rename = "booker_required")]
    pub booker_required: Option<i32>,
    
    /// 下单人手机
    #[serde(rename = "mobile")]
    pub mobile: Option<i32>,
    
}

/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProviderBusinessHour {
    
    /// 结束时间
    #[serde(rename = "close_at")]
    pub close_at: Option<String>,
    
    /// 开始时间
    #[serde(rename = "open_at")]
    pub open_at: Option<String>,
    
    /// 描述
    #[serde(rename = "time_info")]
    pub time_info: Option<String>,
    
}

/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ValidLimitation {
    
    /// 天数内有效
    #[serde(rename = "days_time")]
    pub days_time: Option<i32>,
    
    /// 结束时间
    #[serde(rename = "end_time")]
    pub end_time: Option<i64>,
    
    /// 开始时间
    #[serde(rename = "start_time")]
    pub start_time: Option<i64>,
    
    /// 有效期时间类型
    #[serde(rename = "time_type")]
    pub time_type: Option<i32>,
    
}

/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTicketSkuRuleEdit {
    
    /// 下单人信息设置
    #[serde(rename = "booker_info_limitation")]
    pub booker_info_limitation: Option<BookerInfoLimitation>,
    
    /// 预定须知
    #[serde(rename = "booking_notice")]
    pub booking_notice: Option<BookingNotice>,
    
    /// 下单限制
    #[serde(rename = "order_limitation")]
    pub order_limitation: Option<OrderLimitation>,
    
    /// 商户rule ID
    #[serde(rename = "out_rule_id")]
    pub out_rule_id: Option<String>,
    
    /// 服务商联系方式
    #[serde(rename = "provider_contact_info")]
    pub provider_contact_info: Option<ProviderContactInfo>,
    
    /// 退款规则
    #[serde(rename = "refund_limitations")]
    pub refund_limitations: Option<RefundLimitations>,
    
    /// 拼多多 rule ID
    #[serde(rename = "rule_id")]
    pub rule_id: Option<String>,
    
    /// 商户rule 名称
    #[serde(rename = "rule_name")]
    pub rule_name: Option<String>,
    
    /// 规则版本
    #[serde(rename = "rule_version")]
    pub rule_version: Option<String>,
    
    /// 游玩人信息
    #[serde(rename = "traveler_info_limitation")]
    pub traveler_info_limitation: Option<TravelerInfoLimitation>,
    
    /// 卡券有效期设置
    #[serde(rename = "valid_limitation")]
    pub valid_limitation: Option<ValidLimitation>,
    
}

/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BookingNotice {
    
    /// 入园地址
    #[serde(rename = "enter_address")]
    pub enter_address: Option<String>,
    
    /// 入园时间
    #[serde(rename = "enter_time")]
    pub enter_time: Option<Vec<EnterTime>>,
    
    /// 入园方式
    #[serde(rename = "enter_ways")]
    pub enter_ways: Option<String>,
    
    /// 补充说明
    #[serde(rename = "extra_desc")]
    pub extra_desc: Option<String>,
    
    /// 费用包含
    #[serde(rename = "fee_include")]
    pub fee_include: Option<String>,
    
    /// 费用不包含
    #[serde(rename = "fee_not_include")]
    pub fee_not_include: Option<String>,
    
    /// 重要提示
    #[serde(rename = "important_notice")]
    pub important_notice: Option<String>,
    
    /// 通关限制时间
    #[serde(rename = "pass_time_limit")]
    pub pass_time_limit: Option<i32>,
    
    /// 换票地址
    #[serde(rename = "ticket_place")]
    pub ticket_place: Option<String>,
    
    /// 换票时间
    #[serde(rename = "ticket_time")]
    pub ticket_time: Option<Vec<TicketTime>>,
    
}

/// 修改商品的履约规则
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EnterTime {
    
    /// 备注
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    
    /// 入园结束时间
    #[serde(rename = "end_at")]
    pub end_at: Option<String>,
    
    /// 入园开始时间
    #[serde(rename = "start_at")]
    pub start_at: Option<String>,
    
}


/// 修改商品的履约规则
impl Request for PddTicketSkuRuleEdit {
    fn get_type() -> String {
        "pdd.ticket.sku.rule.edit".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
