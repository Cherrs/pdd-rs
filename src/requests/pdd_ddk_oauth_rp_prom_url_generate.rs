use crate::Request;

use serde::{Deserialize, Serialize};


/// 生成营销工具推广链接
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DiyRedPacketParam {
    
    /// 红包金额列表，200、300、500、1000、2000，单位分。红包金额和红包抵后价设置只能二选一，默认设置了红包金额会忽略红包抵后价设置
    #[serde(rename = "amount_probability")]
    pub amount_probability: Option<Vec<i64>>,
    
    /// 设置玩法，false-现金红包, true-现金券
    #[serde(rename = "dis_text")]
    pub dis_text: Option<bool>,
    
    /// 推广页设置，false-红包开启页, true-红包领取页
    #[serde(rename = "not_show_background")]
    pub not_show_background: Option<bool>,
    
    /// 优先展示类目
    #[serde(rename = "opt_id")]
    pub opt_id: Option<i32>,
    
    /// 自定义红包抵后价和商品佣金区间对象数组
    #[serde(rename = "range_items")]
    pub range_items: Option<Vec<RangeItems>>,
    
}

/// 生成营销工具推广链接
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DiyOneYuanParam {
    
    /// 商品goodsSign，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    #[serde(rename = "goods_sign")]
    pub goods_sign: Option<String>,
    
}

/// 生成营销工具推广链接
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RangeItems {
    
    /// 区间的开始值
    #[serde(rename = "range_from")]
    pub range_from: Option<i64>,
    
    /// range_id为1表示红包抵后价（单位分）， range_id为2表示佣金比例（单位千分之几)
    #[serde(rename = "range_id")]
    pub range_id: Option<i32>,
    
    /// 区间的结束值
    #[serde(rename = "range_to")]
    pub range_to: Option<i64>,
    
}

/// 生成营销工具推广链接
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkOauthRpPromUrlGenerate {
    
    /// 初始金额（单位分），有效金额枚举值：300、500、700、1100和1600，默认300
    #[serde(rename = "amount")]
    pub amount: Option<i64>,
    
    /// 营销工具类型，必填：-1-活动列表，0-红包(需申请推广权限)，2–新人红包，3-刮刮卡，5-员工内购，10-生成绑定备案链接，12-砸金蛋，14-千万补贴B端页面，15-充值中心B端页面，16-千万补贴C端页面，17-千万补贴投票页面，23-超级红包，24-礼金全场N折活动B端页面，27-带货赢千万，28-满减券活动B端页面，29-满减券活动C端页面，30-免单B端页面，31-免单C端页面，32-转盘得现金B端页面，33-转盘得现金C端页面，34-千万神券C端页面，35-千万神券B端页面，36-爆品日历B端页面，37-超级红包B端推品页；红包推广权限申请流程链接：https://jinbao.pinduoduo.com/qa-system?questionId=289，39-母婴馆C端页面，40-母婴馆B端页面
    #[serde(rename = "channel_type")]
    pub channel_type: Option<i32>,
    
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。（如果使用GET请求，请使用URLEncode处理参数）
    #[serde(rename = "custom_parameters")]
    pub custom_parameters: Option<String>,
    
    /// 一元购自定义参数，json格式，例如:{"goods_sign":"Y9b2_0uSWMFPGSaVwvfZAlm_y2ADLWZl_JQ7UYaS80K"}
    #[serde(rename = "diy_one_yuan_param")]
    pub diy_one_yuan_param: Option<DiyOneYuanParam>,
    
    /// 红包自定义参数，json格式
    #[serde(rename = "diy_red_packet_param")]
    pub diy_red_packet_param: Option<DiyRedPacketParam>,
    
    /// 是否生成qq小程序
    #[serde(rename = "generate_qq_app")]
    pub generate_qq_app: Option<bool>,
    
    /// 是否返回 schema URL
    #[serde(rename = "generate_schema_url")]
    pub generate_schema_url: Option<bool>,
    
    /// 是否生成短链接。true-是，false-否，默认false
    #[serde(rename = "generate_short_url")]
    pub generate_short_url: Option<bool>,
    
    /// 是否生成拼多多福利券微信小程序推广信息
    #[serde(rename = "generate_we_app")]
    pub generate_we_app: Option<bool>,
    
    /// 推广位列表，长度最大为1，例如：["60005_612"]。活动页生链要求传入授权备案信息，不支持批量生链。
    #[serde(rename = "p_id_list")]
    pub p_id_list: Option<Vec<String>>,
    
    /// 刮刮卡指定金额（单位分），可指定2-100元间数值，即有效区间为：[200,10000]
    #[serde(rename = "scratch_card_amount")]
    pub scratch_card_amount: Option<i64>,
    
}


impl Request for PddDdkOauthRpPromUrlGenerate {
    fn get_type() -> String {
        "pdd.ddk.oauth.rp.prom.url.generate".to_string()
    }

    fn get_response_name() -> String {
        "rp_promotion_url_generate_response".to_string()
    }
}
