use crate::Request;

use serde::{Deserialize, Serialize};


/// 日历库存类商品编辑或新增价格日历接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChildSkus {
    
    /// 售卖日期（“yyyy-MM-dd”）
    #[serde(rename = "date")]
    pub date: Option<String>,
    
    /// 团购价
    #[serde(rename = "group_price")]
    pub group_price: Option<i64>,
    
    /// 库存增减
    #[serde(rename = "quantity_delta")]
    pub quantity_delta: Option<i64>,
    
    /// 单买价
    #[serde(rename = "single_price")]
    pub single_price: Option<i64>,
    
}

/// 日历库存类商品编辑或新增价格日历接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsChildSkuEdit {
    
    /// 草稿id（未填写则新建一条商品草稿）
    #[serde(rename = "goods_commit_id")]
    pub goods_commit_id: Option<i64>,
    
    /// 商品id
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 日历库存型商品sku信息列表
    #[serde(rename = "skus")]
    pub skus: Option<Vec<Skus>>,
    
    /// 提交后上下架状态，0=上架；1=保持原样。表示编辑商品并提交后商品的上下架状态，不传时默认为0，上架。
    #[serde(rename = "sync_goods_operate")]
    pub sync_goods_operate: Option<i32>,
    
}

/// 日历库存类商品编辑或新增价格日历接口
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Skus {
    
    /// 日历库存商品子sku信息列表
    #[serde(rename = "child_skus")]
    pub child_skus: Option<Vec<ChildSkus>>,
    
    /// 上架状态。0=已下架，1=已上架。不传表示不做修改
    #[serde(rename = "is_onsale")]
    pub is_onsale: Option<i32>,
    
    /// 与sku_id必填其一，用于确定编辑的sku，当有多个sku的out_sku_sn一样时会编辑失败。
    #[serde(rename = "out_sku_sn")]
    pub out_sku_sn: Option<String>,
    
    /// 日历库存商品父skuId
    #[serde(rename = "sku_id")]
    pub sku_id: Option<i64>,
    
}


/// 日历库存类商品编辑或新增价格日历接口
impl Request for PddGoodsChildSkuEdit {
    fn get_type() -> String {
        "pdd.goods.child.sku.edit".to_string()
    }

    fn get_response_name() -> String {
        "child_sku_edit_response".to_string()
    }
}
