use crate::Request;

use serde::{Deserialize, Serialize};


/// 批量启动或暂停创意
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitCreativeUpdateDataOperateStatus {
    
    /// 数据操作状态。1表示开启，2表示暂停。
    #[serde(rename = "dataOperateStatus")]
    pub data_operate_status: Option<i32>,
    
    /// 创意单元Id列表
    #[serde(rename = "unitCreativeIds")]
    pub unit_creative_ids: Option<Vec<i64>>,
    
}


/// 批量启动或暂停创意
impl Request for PddAdApiUnitCreativeUpdateDataOperateStatus {
    fn get_type() -> String {
        "pdd.ad.api.unit.creative.update.data.operate.status".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
