use crate::Request;

use serde::{Deserialize, Serialize};


/// 生成普通商品推广链接
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkOauthGoodsPromUrlGenerate {
    
    /// 多多礼金ID
    #[serde(rename = "cash_gift_id")]
    pub cash_gift_id: Option<i64>,
    
    /// 自定义礼金标题，用于向用户展示渠道专属福利，不超过12个字
    #[serde(rename = "cash_gift_name")]
    pub cash_gift_name: Option<String>,
    
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。（如果使用GET请求，请使用URLEncode处理参数）
    #[serde(rename = "custom_parameters")]
    pub custom_parameters: Option<String>,
    
    /// 是否使用多多客专属推广计划
    #[serde(rename = "force_duo_id")]
    pub force_duo_id: Option<bool>,
    
    /// 是否生成带授权的单品链接。如果未授权，则会走授权流程
    #[serde(rename = "generate_authority_url")]
    pub generate_authority_url: Option<bool>,
    
    /// 是否生成店铺收藏券推广链接
    #[serde(rename = "generate_mall_collect_coupon")]
    pub generate_mall_collect_coupon: Option<bool>,
    
    /// 是否生成qq小程序
    #[serde(rename = "generate_qq_app")]
    pub generate_qq_app: Option<bool>,
    
    /// 是否返回 schema URL
    #[serde(rename = "generate_schema_url")]
    pub generate_schema_url: Option<bool>,
    
    /// 是否生成短链接，true-是，false-否
    #[serde(rename = "generate_short_url")]
    pub generate_short_url: Option<bool>,
    
    /// 是否生成拼多多福利券微信小程序推广信息
    #[serde(rename = "generate_we_app")]
    pub generate_we_app: Option<bool>,
    
    /// 商品goodsSign列表，例如：["c9r2omogKFFAc7WBwvbZU1ikIb16_J3CTa8HNN"]，支持批量生链。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    #[serde(rename = "goods_sign_list")]
    pub goods_sign_list: Option<Vec<String>>,
    
    /// 素材ID，可以通过商品详情接口获取商品素材信息
    #[serde(rename = "material_id")]
    pub material_id: Option<String>,
    
    /// true--生成多人团推广链接 false--生成单人团推广链接（默认false）1、单人团推广链接：用户访问单人团推广链接，可直接购买商品无需拼团。2、多人团推广链接：用户访问双人团推广链接开团，若用户分享给他人参团，则开团者和参团者的佣金均结算给推手
    #[serde(rename = "multi_group")]
    pub multi_group: Option<bool>,
    
    /// 推广位ID
    #[serde(rename = "p_id")]
    pub p_id: Option<String>,
    
    /// 搜索id，建议填写，提高收益。来自pdd.ddk.goods.recommend.get、pdd.ddk.goods.search、pdd.ddk.top.goods.list.query等接口
    #[serde(rename = "search_id")]
    pub search_id: Option<String>,
    
    /// 招商多多客ID
    #[serde(rename = "zs_duo_id")]
    pub zs_duo_id: Option<i64>,
    
}


impl Request for PddDdkOauthGoodsPromUrlGenerate {
    fn get_type() -> String {
        "pdd.ddk.oauth.goods.prom.url.generate".to_string()
    }

    fn get_response_name() -> String {
        "goods_promotion_url_generate_response".to_string()
    }
}
