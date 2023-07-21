use crate::Request;

use serde::{Deserialize, Serialize};


/// 查询店铺身份
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddFdsRoleGet {
    
}


impl Request for PddFdsRoleGet {
    fn get_type() -> String {
        "pdd.fds.role.get".to_string()
    }

    fn get_response_name() -> String {
        "pdd_fds_role_get_response".to_string()
    }
}
