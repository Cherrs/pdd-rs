use crate::Request;

use serde::{Deserialize, Serialize};


/// 运营频道商品查询
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkOauthGoodsRecommendGet {
    
    /// 活动商品标记数组，例：[4,7]，4-秒杀，7-百亿补贴，10851-千万补贴，11879-千万神券，10913-招商礼金商品，31-品牌黑标，10564-精选爆品-官方直推爆款，10584-精选爆品-团长推荐，24-品牌高佣，其他的值请忽略
    #[serde(rename = "activity_tags")]
    pub activity_tags: Option<Vec<i32>>,
    
    /// 猜你喜欢场景的商品类目，20100-百货，20200-母婴，20300-食品，20400-女装，20500-电器，20600-鞋包，20700-内衣，20800-美妆，20900-男装，21000-水果，21100-家纺，21200-文具,21300-运动,21400-虚拟,21500-汽车,21600-家装,21700-家具,21800-医药;
    #[serde(rename = "cat_id")]
    pub cat_id: Option<i64>,
    
    /// 进宝频道推广商品: 1-今日销量榜,3-相似商品推荐,4-猜你喜欢(和进宝网站精选一致),5-实时热销榜,6-实时收益榜。默认值5
    #[serde(rename = "channel_type")]
    pub channel_type: Option<i32>,
    
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 为用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 为上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。
    #[serde(rename = "custom_parameters")]
    pub custom_parameters: Option<String>,
    
    /// 是否使用工具商专属推广计划，默认为false
    #[serde(rename = "force_auth_duo_id")]
    pub force_auth_duo_id: Option<bool>,
    
    /// 商品主图类型：1-场景图，2-白底图，默认为0
    #[serde(rename = "goods_img_type")]
    pub goods_img_type: Option<i32>,
    
    /// 商品goodsSign列表，相似商品推荐场景时必传，仅取数组的第一位，例如：["c9r2omogKFFAc7WBwvbZU1ikIb16_J3CTa8HNN"]。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    #[serde(rename = "goods_sign_list")]
    pub goods_sign_list: Option<Vec<String>>,
    
    /// 一页请求数量；默认值 ： 20
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    
    /// 翻页时建议填写前页返回的list_id值
    #[serde(rename = "list_id")]
    pub list_id: Option<String>,
    
    /// 从多少位置开始请求；默认值 ： 0
    #[serde(rename = "offset")]
    pub offset: Option<i32>,
    
    /// 推广位id
    #[serde(rename = "pid")]
    pub pid: Option<String>,
    
}


/// 运营频道商品查询
impl Request for PddDdkOauthGoodsRecommendGet {
    fn get_type() -> String {
        "pdd.ddk.oauth.goods.recommend.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_basic_detail_response".to_string()
    }
}
