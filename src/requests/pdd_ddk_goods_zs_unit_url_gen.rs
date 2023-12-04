use crate::Request;

use serde::{Deserialize, Serialize};


/// 本功能适用于采集群等场景。将其他推广者的推广链接转换成自己的；通过此api，可以将他人的招商推广链接，转换成自己的招商推广链接。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkGoodsZsUnitUrlGen {
    
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。若进行cid投放，转链的时候不填充custom_parameters，后续在推广前原始链接上拼接custom_parameters。（如果使用GET请求，请使用URLEncode处理参数）
    #[serde(rename = "custom_parameters")]
    pub custom_parameters: Option<String>,
    
    /// 是否生成微信shortlink链接，仅支持单品，单个渠道每天生成的shortLink数量有限，请合理生成shortLink链接
    #[serde(rename = "generate_short_link")]
    pub generate_short_link: Option<bool>,
    
    /// 渠道id
    #[serde(rename = "pid")]
    pub pid: Option<String>,
    
    /// 需转链的链接，支持拼多多商品链接、进宝长链/短链（即为pdd.ddk.goods.promotion.url.generate接口生成的长短链）
    #[serde(rename = "source_url")]
    pub source_url: Option<String>,
    
}


/// 本功能适用于采集群等场景。将其他推广者的推广链接转换成自己的；通过此api，可以将他人的招商推广链接，转换成自己的招商推广链接。
impl Request for PddDdkGoodsZsUnitUrlGen {
    fn get_type() -> String {
        "pdd.ddk.goods.zs.unit.url.gen".to_string()
    }

    fn get_response_name() -> String {
        "goods_zs_unit_generate_response".to_string()
    }
}
