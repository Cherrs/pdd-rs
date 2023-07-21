use std::{borrow::Cow, env};

use md5::{Digest, Md5};
use public_parameters::PublicParameters;
use reqwest::header::CONTENT_TYPE;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;
use time::OffsetDateTime;

mod file;
pub use file::PddFile;
use tracing::trace;

pub mod public_parameters;
pub mod requests;

#[cfg(feature = "pmc-native-tls")]
pub mod pmc_client;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub url: String,
    pub access_token: Option<String>,
}

impl Config {
    /// 从环境变量获取Config
    ///
    /// ## Powershell:
    /// - `$env:PDD_CLIENT_ID = "clientidabaaba"`
    /// - `$env:PDD_CLIENT_SECRET = "clientsecretabaaba"`
    /// - `$env:PDD_URL = "https://gw-api.pinduoduo.com/api/router"`
    /// - `$env:PDD_ACCESS_TOKEN = "pdd_access_tokenabaabaaba"`
    ///
    /// ## Bash
    /// - `export PDD_CLIENT_ID="clientidabaaba"`
    /// - `export PDD_CLIENT_SECRET="clientsecretabaaba"`
    /// - `export PDD_URL="https://gw-api.pinduoduo.com/api/router"`
    /// - `export PDD_ACCESS_TOKEN="pdd_access_tokenabaabaaba"`
    ///
    /// # Examples
    /// `
    /// let config = Config::from_env();
    /// `
    pub fn from_env() -> Result<Self, Error> {
        let access_token = if let Ok(token) = env::var("PDD_ACCESS_TOKEN") {
            Some(token)
        } else {
            None
        };

        let url = if let Ok(url) = env::var("PDD_URL") {
            url
        } else {
            "https://gw-api.pinduoduo.com/api/router".to_string()
        };
        Ok(Config {
            client_id: env::var("PDD_CLIENT_ID")?,
            client_secret: env::var("PDD_CLIENT_SECRET")?,
            access_token,
            url,
        })
    }
}

pub struct Client {
    pub config: Config,
    pub reqwest: reqwest::Client,
}

pub trait Request {
    fn get_type() -> String;

    fn get_response_name() -> String;
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client {
            config,
            reqwest: reqwest::Client::new(),
        }
    }

    pub fn from_env() -> Result<Self, Error> {
        Ok(Client {
            config: Config::from_env()?,
            reqwest: reqwest::Client::new(),
        })
    }

    pub async fn exec<T: Request + Serialize>(self, req: T) -> Result<Value, Error> {
        let pub_par = PublicParameters {
            access_token: self.config.access_token,
            client_id: self.config.client_id,
            data_type: Some("JSON".to_string()),
            timestamp: OffsetDateTime::now_utc().unix_timestamp(),
            type_: T::get_type(),
            version: None,
        };

        let pub_parameters = serde_json::to_value(pub_par)?;

        let req_value = serde_json::to_value(req)?;

        let body = add_sign(&self.config.client_secret, pub_parameters, req_value);

        let rsp = self
            .reqwest
            .post(self.config.url)
            .header(CONTENT_TYPE, "application/json;charset=utf-8")
            .json(&body)
            .send()
            .await?;
        let result = rsp.json::<Value>().await?;

        trace!("{:?}", result);
        Ok(result)
    }
}

pub fn add_sign(client_secret: &str, mut pub_parameters: Value, mut parameters: Value) -> Value {
    let parameters_map = parameters.as_object_mut().unwrap();
    parameters_map.append(pub_parameters.as_object_mut().unwrap());

    let mut pub_parameters_map: Vec<(&String, &Value)> =
        parameters_map.iter().map(|f| (f.0, f.1)).collect();
    pub_parameters_map.sort_by(|a, b| a.0.bytes().cmp(b.0.bytes()));
    let mut sign: Vec<Cow<str>> = Vec::new();

    sign.push(Cow::Borrowed(client_secret));
    for (k, v) in pub_parameters_map {
        let value = match v {
            Value::String(s) => Cow::Borrowed(s.as_str()),
            Value::Null => continue,
            _ => Cow::Owned(v.to_string()),
        };
        sign.push(Cow::Borrowed(k));
        sign.push(value.clone());
    }
    sign.push(Cow::Borrowed(client_secret));

    let mut h = Md5::new();
    h.update(sign.concat());
    let md5 = h.finalize();

    parameters_map.insert("sign".to_string(), Value::String(format!("{:X}", md5)));

    Value::Object(parameters_map.to_owned())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("使用环境变量初始化Client失败")]
    EnvVarNotFoundError(#[from] env::VarError),
    #[error("Json解析错误")]
    JsonParsingFailure(#[from] serde_json::Error),
    #[error("请求失败")]
    RequestFailure(#[from] reqwest::Error),
    #[cfg(feature = "pmc-native-tls")]
    #[error("消息服务连接失败")]
    PmcConnectionFailure,
    #[cfg(feature = "pmc-native-tls")]
    #[error("发生错误")]
    PmcFailure,
    #[cfg(feature = "pmc-native-tls")]
    #[error("消息解析错误")]
    PmcMessageFailure,
}

#[cfg(test)]
mod tests {
    use crate::requests::PddOrderInformationGet;

    use super::*;

    #[tokio::test]
    async fn exec_test() {
        let client = Client::from_env().unwrap();
        let req = PddOrderInformationGet {
            order_sn: Some("230626-434073824910838".to_string()),
        };
        client.exec(req).await.unwrap();
    }

    #[test]
    fn build_sign_test() {}
}
