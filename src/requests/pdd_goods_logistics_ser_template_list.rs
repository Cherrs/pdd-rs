use crate::Request;

use serde::{Deserialize, Serialize};


/// 商品送装服务模版列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsLogisticsSerTemplateList {
    
    /// 查询大小
    #[serde(rename = "length")]
    pub length: Option<i32>,
    
    /// 查询类型
    #[serde(rename = "query_type")]
    pub query_type: Option<i32>,
    
    /// 查询偏移量
    #[serde(rename = "start")]
    pub start: Option<i32>,
    
    /// 模板类型
    #[serde(rename = "template_type")]
    pub template_type: Option<i32>,
    
}


impl Request for PddGoodsLogisticsSerTemplateList {
    fn get_type() -> String {
        "pdd.goods.logistics.ser.template.list".to_string()
    }

    fn get_response_name() -> String {
        "goods_logistics_ser_template_list_response".to_string()
    }
}
