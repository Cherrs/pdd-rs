use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品送装服务模版详情
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsLogisticsSerTemplateDetail {
    
    /// 模版id
    #[serde(rename = "template_id")]
    pub template_id: Option<String>,
    
}


/// 商品送装服务模版详情
impl Request for PddGoodsLogisticsSerTemplateDetail {
    fn get_type() -> String {
        "pdd.goods.logistics.ser.template.detail".to_string()
    }

    fn get_response_name() -> String {
        "goods_logistics_ser_template_detail_response".to_string()
    }
}
