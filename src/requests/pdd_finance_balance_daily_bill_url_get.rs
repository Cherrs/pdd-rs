use crate::Request;

use serde::{Deserialize, Serialize};


/// 获取商家货款日账单下载链接
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFinanceBalanceDailyBillUrlGet {
    
    /// 账单日期（形如yyyy-MM-dd）；例如入参为“2019-03-24”，则返回2019年3月24日的商家货款日账单的下载链接
    #[serde(rename = "bill_date")]
    pub bill_date: Option<String>,
    
}


impl Request for PddFinanceBalanceDailyBillUrlGet {
    fn get_type() -> String {
        "pdd.finance.balance.daily.bill.url.get".to_string()
    }

    fn get_response_name() -> String {
        "finance_balance_daily_bill_url_get_response".to_string()
    }
}
