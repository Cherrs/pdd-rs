use crate::Request;

use serde::{Deserialize, Serialize};


/// 批量启停全站推广广告注：同一店铺内所有全站推广相关的写接口不可并发调用
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitTrUpdateDataOperateStatus {
    
    /// 商家操作状态：1-启动 2-暂停
    #[serde(rename = "dataOperateStatus")]
    pub data_operate_status: Option<i32>,
    
    /// 商品id列表
    #[serde(rename = "goodsIds")]
    pub goods_ids: Option<Vec<i64>>,
    
}


impl Request for PddAdApiUnitTrUpdateDataOperateStatus {
    fn get_type() -> String {
        "pdd.ad.api.unit.tr.update.data.operate.status".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
