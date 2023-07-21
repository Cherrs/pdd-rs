use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品送装服务模版删除
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsLogisticsSerTemplateDelete {
    
    /// 模版id
    #[serde(rename = "template_id")]
    pub template_id: Option<String>,
    
}


impl Request for PddGoodsLogisticsSerTemplateDelete {
    fn get_type() -> String {
        "pdd.goods.logistics.ser.template.delete".to_string()
    }

    fn get_response_name() -> String {
        "goods_logistics_ser_template_delete_response".to_string()
    }
}
