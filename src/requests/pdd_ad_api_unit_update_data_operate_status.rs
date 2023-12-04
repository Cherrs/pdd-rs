use crate::Request;

use serde::{Deserialize, Serialize};


/// 批量启动或暂停单元
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiUnitUpdateDataOperateStatus {
    
    /// 广告单元Id列表。一次不得超过20个。
    #[serde(rename = "adIds")]
    pub ad_ids: Option<Vec<i64>>,
    
    /// 数据操作状态。1表示开启，2表示暂停。
    #[serde(rename = "dataOperateStatus")]
    pub data_operate_status: Option<i32>,
    
}


/// 批量启动或暂停单元
impl Request for PddAdApiUnitUpdateDataOperateStatus {
    fn get_type() -> String {
        "pdd.ad.api.unit.update.data.operate.status".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
