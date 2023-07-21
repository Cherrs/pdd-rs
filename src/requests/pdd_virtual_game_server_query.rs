use crate::Request;

use serde::{Deserialize, Serialize};


/// 虚拟游戏类区服列表
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddVirtualGameServerQuery {
    
    /// 游戏CODE
    #[serde(rename = "goods_config_code")]
    pub goods_config_code: Option<String>,
    
}


impl Request for PddVirtualGameServerQuery {
    fn get_type() -> String {
        "pdd.virtual.game.server.query".to_string()
    }

    fn get_response_name() -> String {
        "game_server_query_response".to_string()
    }
}
