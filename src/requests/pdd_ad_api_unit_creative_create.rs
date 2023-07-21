use crate::Request;

use serde::{Deserialize, Serialize};


/// 创建创意
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdCreativeCreateMessage {
    
    /// 创意图片列表
    #[serde(rename = "adImageVOList")]
    pub ad_image_vo_list: Option<Vec<AdImageVoList>>,
    
    /// 创意标题列表
    #[serde(rename = "adTextVOList")]
    pub ad_text_vo_list: Option<Vec<AdTextVoList>>,
    
    /// 创意规格，6：商品轮播图，7：商品长图，其余规格暂不支持
    #[serde(rename = "creativeSpecificationId")]
    pub creative_specification_id: Option<i64>,
    
}

/// 创建创意
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitCreativeCreate {
    
    /// 创意列表
    #[serde(rename = "adCreativeCreateMessage")]
    pub ad_creative_create_message: Option<AdCreativeCreateMessage>,
    
    /// 广告单元Id
    #[serde(rename = "adId")]
    pub ad_id: Option<i64>,
    
}

/// 创建创意
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdTextVoList {
    
    /// 标题
    #[serde(rename = "text")]
    pub text: Option<String>,
    
}

/// 创建创意
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdImageVoList {
    
    /// 图片链接，可用图片参考以下接口返回：pdd.ad.api.goods.query.gallery.images（轮播图），pdd.ad.api.goods.query.long.images（长图）
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    
}


impl Request for PddAdApiUnitCreativeCreate {
    fn get_type() -> String {
        "pdd.ad.api.unit.creative.create".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
