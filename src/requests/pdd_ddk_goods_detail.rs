use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询多多进宝商品详情
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkGoodsDetail {
    
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。（如果使用GET请求，请使用URLEncode处理参数）
    #[serde(rename = "custom_parameters")]
    pub custom_parameters: Option<String>,
    
    /// 商品主图类型：1-场景图，2-白底图，默认为0
    #[serde(rename = "goods_img_type")]
    pub goods_img_type: Option<i32>,
    
    /// 商品goodsSign，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    #[serde(rename = "goods_sign")]
    pub goods_sign: Option<String>,
    
    /// 是否获取sku信息，默认false不返回。（特殊渠道权限，需额外申请）
    #[serde(rename = "need_sku_info")]
    pub need_sku_info: Option<bool>,
    
    /// 推广位id
    #[serde(rename = "pid")]
    pub pid: Option<String>,
    
    /// 搜索id，建议填写，提高收益。来自pdd.ddk.goods.recommend.get、pdd.ddk.goods.search、pdd.ddk.top.goods.list.query等接口
    #[serde(rename = "search_id")]
    pub search_id: Option<String>,
    
    /// 招商多多客ID
    #[serde(rename = "zs_duo_id")]
    pub zs_duo_id: Option<i64>,
    
}


/// 查询多多进宝商品详情
impl Request for PddDdkGoodsDetail {
    fn get_type() -> String {
        "pdd.ddk.goods.detail".to_string()
    }

    fn get_response_name() -> String {
        "goods_detail_response".to_string()
    }
}
