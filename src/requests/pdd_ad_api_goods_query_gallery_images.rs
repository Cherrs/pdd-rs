use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询商品轮播图
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiGoodsQueryGalleryImages {
    
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
}


impl Request for PddAdApiGoodsQueryGalleryImages {
    fn get_type() -> String {
        "pdd.ad.api.goods.query.gallery.images".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
