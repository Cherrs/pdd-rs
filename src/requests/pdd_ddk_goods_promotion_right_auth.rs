use crate::Request;

use serde::{Deserialize, Serialize};


/// 多多进宝信息流渠道备案授权素材上传使用
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddDdkGoodsPromotionRightAuth {
    
    /// 推广商品视频素材url
    #[serde(rename = "demo_url")]
    pub demo_url: Option<String>,
    
    /// 渠道duoId
    #[serde(rename = "duo_id")]
    pub duo_id: Option<i64>,
    
    /// 商品GoodsId
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 商家资质证明图片url列表，1到3张图
    #[serde(rename = "mall_certificate_url")]
    pub mall_certificate_url: Option<Vec<String>>,
    
    /// 推广视频预览码url
    #[serde(rename = "promotion_code_url")]
    pub promotion_code_url: Option<String>,
    
    /// 推广结束时间戳，毫秒
    #[serde(rename = "promotion_end_time")]
    pub promotion_end_time: Option<i64>,
    
    /// 推广开始时间戳，毫秒
    #[serde(rename = "promotion_start_time")]
    pub promotion_start_time: Option<i64>,
    
    /// 商品图片素材url列表，0到3张图
    #[serde(rename = "thumb_pic_url")]
    pub thumb_pic_url: Option<Vec<String>>,
    
}


/// 多多进宝信息流渠道备案授权素材上传使用
impl Request for PddDdkGoodsPromotionRightAuth {
    fn get_type() -> String {
        "pdd.ddk.goods.promotion.right.auth".to_string()
    }

    fn get_response_name() -> String {
        "goods_promotion_right_auth_response".to_string()
    }
}
