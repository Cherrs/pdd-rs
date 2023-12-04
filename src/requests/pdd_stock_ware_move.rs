use crate::Request;

use serde::{Deserialize, Serialize};

/// 家电分仓库存-库存信息调整
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StockMoveOrderActionDto {
    /// 调整方向。1, "入库"；2, "出库"；3, "库存同步"
    #[serde(rename = "move_direction")]
    pub move_direction: Option<i32>,

    /// 调整单备注
    #[serde(rename = "order_note")]
    pub order_note: Option<String>,

    /// 业务类型。1, "采购"；2, "调拨"；3, "退货"；4, "盘点"；5, "发货"；6, "库存同步"
    #[serde(rename = "business_type")]
    pub business_type: Option<i32>,

    /// 仓库编码
    #[serde(rename = "warehouse_sn")]
    pub warehouse_sn: Option<String>,

    /// 调整时间
    #[serde(rename = "move_time")]
    pub move_time: Option<i64>,

    /// 调整单号
    #[serde(rename = "move_order_sn")]
    pub move_order_sn: Option<String>,
}

/// 家电分仓库存-库存信息调整
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddStockWareMove {
    ///
    #[serde(rename = "stock_move_order_action_dto")]
    pub stock_move_order_action_dto: Option<StockMoveOrderActionDto>,

    /// List<JsonObject>的json string, 一次传入StockMoveRecordActionDTO list size不超过30个
    #[serde(rename = "stock_move_record_action_dto_list")]
    pub stock_move_record_action_dto_list: Option<Vec<StockMoveRecordActionDtoList>>,
}

/// 家电分仓库存-库存信息调整
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StockMoveRecordActionDtoList {
    /// 备注
    #[serde(rename = "note")]
    pub note: Option<String>,

    /// 调整数量
    #[serde(rename = "move_num")]
    pub move_num: Option<i64>,

    /// 货品sn
    #[serde(rename = "ware_sn")]
    pub ware_sn: Option<String>,
}

/// 家电分仓库存-库存信息调整
impl Request for PddStockWareMove {
    fn get_type() -> String {
        "pdd.stock.ware.move".to_string()
    }

    fn get_response_name() -> String {
        "open_api_response".to_string()
    }
}
