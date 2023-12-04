use crate::Request;

use serde::{Deserialize, Serialize};


/// 启动或暂停计划
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddAdApiPlanUpdateDataOperateStatus {
    
    /// 数据操作状态。1表示开启，2表示暂停。
    #[serde(rename = "dataOperateStatus")]
    pub data_operate_status: Option<i32>,
    
    /// 广告计划Id列表
    #[serde(rename = "planIds")]
    pub plan_ids: Option<Vec<i64>>,
    
    /// 场景类型。0表示搜索，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: Option<i32>,
    
}


/// 启动或暂停计划
impl Request for PddAdApiPlanUpdateDataOperateStatus {
    fn get_type() -> String {
        "pdd.ad.api.plan.update.data.operate.status".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
