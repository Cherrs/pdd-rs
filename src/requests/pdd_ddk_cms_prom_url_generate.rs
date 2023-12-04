use crate::Request;

use serde::{Deserialize, Serialize};


/// 生成商城推广链接接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkCmsPromUrlGenerate {
    
    /// 0, "1.9包邮"；1, "今日爆款"； 2, "品牌清仓"； 4,"PC端专属商城(已下线，会生成默认商城链接)"；不传值为默认商城
    #[serde(rename = "channel_type")]
    pub channel_type: Option<i32>,
    
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。（如果使用GET请求，请使用URLEncode处理参数）
    #[serde(rename = "custom_parameters")]
    pub custom_parameters: Option<String>,
    
    /// 是否生成手机跳转链接。true-是，false-否，默认false
    #[serde(rename = "generate_mobile")]
    pub generate_mobile: Option<bool>,
    
    /// 是否返回 schema URL
    #[serde(rename = "generate_schema_url")]
    pub generate_schema_url: Option<bool>,
    
    /// 是否生成短链接，true-是，false-否
    #[serde(rename = "generate_short_url")]
    pub generate_short_url: Option<bool>,
    
    /// 是否生成拼多多福利券微信小程序推广信息
    #[serde(rename = "generate_we_app")]
    pub generate_we_app: Option<bool>,
    
    /// 搜索关键词
    #[serde(rename = "keyword")]
    pub keyword: Option<String>,
    
    /// 单人团多人团标志。true-多人团，false-单人团 默认false
    #[serde(rename = "multi_group")]
    pub multi_group: Option<bool>,
    
    /// 推广位列表，例如：["60005_612"]
    #[serde(rename = "p_id_list")]
    pub p_id_list: Option<Vec<String>>,
    
}


/// 生成商城推广链接接口
impl Request for PddDdkCmsPromUrlGenerate {
    fn get_type() -> String {
        "pdd.ddk.cms.prom.url.generate".to_string()
    }

    fn get_response_name() -> String {
        "cms_promotion_url_generate_response".to_string()
    }
}
