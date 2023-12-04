use crate::Request;

use serde::{Deserialize, Serialize};


/// 供应商批量添加卡券
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DataList {
    
    /// 卡密卡号，商家卡密必填
    #[serde(rename = "cardNo")]
    pub card_no: Option<String>,
    
    /// 用户核销卡密加密串，加密所使用public key向对接小二获取（加密算法"RSA"，填充方式"RSA/ECB/PKCS1Padding"）
    #[serde(rename = "encryptPassword")]
    pub encrypt_password: Option<String>,
    
}

/// 供应商批量添加卡券
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVoucherVirtualCardBatchAdd {
    
    /// 业务数据
    #[serde(rename = "data")]
    pub data: Option<Data>,
    
}

/// 供应商批量添加卡券
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Data {
    
    /// 充值地址
    #[serde(rename = "chargeAddress")]
    pub charge_address: Option<String>,
    
    /// 卡密信息列表，一次请求最多5000条卡密
    #[serde(rename = "dataList")]
    pub data_list: Option<Vec<DataList>>,
    
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
    
    /// skuId
    #[serde(rename = "skuId")]
    pub sku_id: Option<i64>,
    
}


/// 供应商批量添加卡券
impl Request for PddVoucherVirtualCardBatchAdd {
    fn get_type() -> String {
        "pdd.voucher.virtual.card.batch.add".to_string()
    }

    fn get_response_name() -> String {
        "response".to_string()
    }
}
