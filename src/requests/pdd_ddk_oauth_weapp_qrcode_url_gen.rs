use crate::Request;

use serde::{Deserialize, Serialize};


/// 多多客工具生成单品推广小程序二维码url
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkOauthWeappQrcodeUrlGen {
    
    /// 自定义参数，为链接打上自定义标签。自定义参数最长限制64个字节。
    #[serde(rename = "custom_parameters")]
    pub custom_parameters: Option<String>,
    
    /// 商品ID，仅支持单个查询
    #[serde(rename = "goods_id_list")]
    pub goods_id_list: Option<Vec<i64>>,
    
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


impl Request for PddDdkOauthWeappQrcodeUrlGen {
    fn get_type() -> String {
        "pdd.ddk.oauth.weapp.qrcode.url.gen".to_string()
    }

    fn get_response_name() -> String {
        "weapp_qrcode_generate_response".to_string()
    }
}
