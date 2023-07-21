use crate::Request;

use serde::{Deserialize, Serialize};


/// 多多客生成单品推广小程序二维码url
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkWeappQrcodeUrlGen {
    
    /// 多多礼金ID
    #[serde(rename = "cash_gift_id")]
    pub cash_gift_id: Option<i64>,
    
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key
    #[serde(rename = "custom_parameters")]
    pub custom_parameters: Option<String>,
    
    /// 是否生成店铺收藏券推广链接
    #[serde(rename = "generate_mall_collect_coupon")]
    pub generate_mall_collect_coupon: Option<bool>,
    
    /// 商品goodsSign列表，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    #[serde(rename = "goods_sign_list")]
    pub goods_sign_list: Option<Vec<String>>,
    
    /// 推广位ID
    #[serde(rename = "p_id")]
    pub p_id: Option<String>,
    
    /// 招商多多客ID
    #[serde(rename = "zs_duo_id")]
    pub zs_duo_id: Option<i64>,
    
}


impl Request for PddDdkWeappQrcodeUrlGen {
    fn get_type() -> String {
        "pdd.ddk.weapp.qrcode.url.gen".to_string()
    }

    fn get_response_name() -> String {
        "weapp_qrcode_generate_response".to_string()
    }
}
