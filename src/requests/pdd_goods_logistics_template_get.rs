use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取拼多多商家的物流运费模板信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsLogisticsTemplateGet {
    
    /// 默认返回运费模板的页数为1，最高为100页，注意：page与page_size必须传一个
    #[serde(rename = "page")]
    pub page: Option<i32>,
    
    /// 默认返回20条模板数据，最多100条数据
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    
    /// 0-新发布商品，1-编辑商品。如传值为空，平台默认为发布商品
    #[serde(rename = "goods_status")]
    pub goods_status: Option<i32>,
    
}


impl Request for PddGoodsLogisticsTemplateGet {
    fn get_type() -> String {
        "pdd.goods.logistics.template.get".to_string()
    }

    fn get_response_name() -> String {
        "goods_logistics_template_get_response".to_string()
    }
}
