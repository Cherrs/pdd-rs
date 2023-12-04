use crate::Request;

use serde::{Deserialize, Serialize};


/// 更新创意内容
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdCreativeUpdateMessage {
    
    /// 创意图片列表
    #[serde(rename = "adImageVOList")]
    pub ad_image_vo_list: Option<Vec<AdImageVoList>>,
    
    /// 创意标题列表
    #[serde(rename = "adTextVOList")]
    pub ad_text_vo_list: Option<Vec<AdTextVoList>>,
    
    /// 创意规格Id，6-轮播图，7-长图
    #[serde(rename = "creativeSpecificationId")]
    pub creative_specification_id: Option<i64>,
    
}

/// 更新创意内容
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitCreativeUpdateContent {
    
    /// 创意更新输入
    #[serde(rename = "adCreativeUpdateMessage")]
    pub ad_creative_update_message: Option<AdCreativeUpdateMessage>,
    
    /// 创意单元Id
    #[serde(rename = "unitCreativeId")]
    pub unit_creative_id: Option<i64>,
    
}

/// 更新创意内容
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdTextVoList {
    
    /// 标题文字
    #[serde(rename = "text")]
    pub text: Option<String>,
    
}

/// 更新创意内容
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AdImageVoList {
    
    /// 图片链接，可用图片参考以下接口返回：pdd.ad.api.goods.query.gallery.images（轮播图），pdd.ad.api.goods.query.long.images（长图）
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    
}


/// 更新创意内容
impl Request for PddAdApiUnitCreativeUpdateContent {
    fn get_type() -> String {
        "pdd.ad.api.unit.creative.update.content".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
