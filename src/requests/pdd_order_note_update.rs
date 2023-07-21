use crate::Request;

use serde::{Deserialize, Serialize};


/// 编辑商家订单备注信息
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddOrderNoteUpdate {
    
    /// 订单备注
    #[serde(rename = "note")]
    pub note: Option<String>,
    
    /// 备注标记：1-红色，2-黄色，3-绿色，4-蓝色，5-紫色，tag与tag_name关联，都入参或都不入参
    #[serde(rename = "tag")]
    pub tag: Option<i32>,
    
    /// 标记名称；长度最大为3个字符，tag与tag_name关联，都入参或都不入参
    #[serde(rename = "tag_name")]
    pub tag_name: Option<String>,
    
    /// 订单号
    #[serde(rename = "order_sn")]
    pub order_sn: Option<String>,
    
}


impl Request for PddOrderNoteUpdate {
    fn get_type() -> String {
        "pdd.order.note.update".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
