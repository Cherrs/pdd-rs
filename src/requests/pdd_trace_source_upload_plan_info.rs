use crate::Request;

use serde::{Deserialize, Serialize};


/// 溯源服务商上传正品溯源粘贴计划, 用于正品溯源功能
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddTraceSourceUploadPlanInfo {
    
    /// 到港日期
    #[serde(rename = "arrive_time")]
    pub arrive_time: Option<String>,
    
    /// 提单号
    #[serde(rename = "bill_no")]
    pub bill_no: Option<String>,
    
    /// 报检日期
    #[serde(rename = "ciq_date")]
    pub ciq_date: Option<String>,
    
    /// 报检单号
    #[serde(rename = "ciq_no")]
    pub ciq_no: Option<String>,
    
    /// 境内收发货人
    #[serde(rename = "dealer_org")]
    pub dealer_org: Option<String>,
    
    /// 申报单位
    #[serde(rename = "declare_org")]
    pub declare_org: Option<String>,
    
    /// 启运地
    #[serde(rename = "desp_port_name")]
    pub desp_port_name: Option<String>,
    
    /// 报关日期
    #[serde(rename = "entry_date")]
    pub entry_date: Option<String>,
    
    /// 报关单号
    #[serde(rename = "entry_no")]
    pub entry_no: Option<String>,
    
    /// 溯源码粘贴计划(商品维度)
    #[serde(rename = "goods")]
    pub goods: Option<Vec<Goods>>,
    
    /// 清单申报日期
    #[serde(rename = "list_date")]
    pub list_date: Option<String>,
    
    /// 核注清单编号
    #[serde(rename = "list_no")]
    pub list_no: Option<String>,
    
    /// 装货港
    #[serde(rename = "load_port")]
    pub load_port: Option<String>,
    
    /// 粘贴计划所属店铺ID
    #[serde(rename = "mall_id")]
    pub mall_id: Option<i64>,
    
    /// 粘贴计划所属店铺名
    #[serde(rename = "mall_name")]
    pub mall_name: Option<String>,
    
    /// 粘贴计划单激活时间
    #[serde(rename = "plan_active_time")]
    pub plan_active_time: Option<String>,
    
    /// 粘贴计划单创建时间
    #[serde(rename = "plan_created_time")]
    pub plan_created_time: Option<String>,
    
    /// 粘贴计划单编号
    #[serde(rename = "plan_no")]
    pub plan_no: Option<String>,
    
    /// 进口口岸
    #[serde(rename = "port")]
    pub port: Option<String>,
    
    /// 运输方式
    #[serde(rename = "transport_mode")]
    pub transport_mode: Option<String>,
    
    /// 粘贴计划单所属保税仓名称
    #[serde(rename = "warehouse_name")]
    pub warehouse_name: Option<String>,
    
}

/// 溯源服务商上传正品溯源粘贴计划, 用于正品溯源功能
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Goods {
    
    /// 防伪溯源码粘贴数量
    #[serde(rename = "code_amount")]
    pub code_amount: Option<i64>,
    
    /// 防伪溯源码结束顺序号
    #[serde(rename = "end_serial_no")]
    pub end_serial_no: Option<String>,
    
    /// 商品ID
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 商品备案图片
    #[serde(rename = "goods_image_url")]
    pub goods_image_url: Option<String>,
    
    /// 商品备案名称
    #[serde(rename = "goods_name")]
    pub goods_name: Option<String>,
    
    /// 原产国(地)
    #[serde(rename = "goods_origin")]
    pub goods_origin: Option<String>,
    
    /// 商品备案规格型号
    #[serde(rename = "goods_property")]
    pub goods_property: Option<String>,
    
    /// 商品规格
    #[serde(rename = "goods_sku_no")]
    pub goods_sku_no: Option<String>,
    
    /// Hs编码
    #[serde(rename = "hs_code")]
    pub hs_code: Option<String>,
    
    /// Hs名称
    #[serde(rename = "hs_name")]
    pub hs_name: Option<String>,
    
    /// 防伪溯源码起始顺序号
    #[serde(rename = "start_serial_no")]
    pub start_serial_no: Option<String>,
    
}


impl Request for PddTraceSourceUploadPlanInfo {
    fn get_type() -> String {
        "pdd.trace.source.upload.plan.info".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
