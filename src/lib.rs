//! # 拼多多开放平台Rust SDK
//!
//! 基于tokio开发的拼多多开发平台sdk，支持消息推送
//!
//! 所有请求可以在[`requests`]里找到
//!
//! 使用消息推送需要开启features `pmc-native-tls`
//!
//! 更多例子看[`examples`](https://github.com/Cherrs/pdd-rs/tree/main/examples)
use std::{borrow::Cow, env};

use file::FileUploadRequest;
use md5::{Digest, Md5};
use reqwest::{
    header::CONTENT_TYPE,
    multipart::{Form, Part},
};
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;
use time::OffsetDateTime;

mod file;
pub use file::PddFile;
use tracing::trace;

mod public_parameters;
pub use public_parameters::PublicParameters;
pub mod requests;

#[cfg(feature = "pmc-native-tls")]
pub mod pmc;

/// PDD的授权信息，[参考](https://open.pinduoduo.com/application/document/browse?idStr=8EC06C399636041E)
pub struct Config {
    /// client_id
    pub client_id: String,
    /// client_secret
    pub client_secret: String,
    /// 拼多多的api请求地址
    pub url: String,
    /// 文件上传的请求地址
    pub upload_url: String,
    /// 获取到的access_token
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
    /// ## Bash:
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

        let upload_url = if let Ok(url) = env::var("PDD_UPLOAD_URL") {
            url
        } else {
            "https://gw-upload.pinduoduo.com/api/upload".to_string()
        };
        Ok(Config {
            client_id: env::var("PDD_CLIENT_ID")?,
            client_secret: env::var("PDD_CLIENT_SECRET")?,
            access_token,
            url,
            upload_url,
        })
    }
}

/// 拼多多开放平台客户端
pub struct Client {
    pub config: Config,
    pub reqwest: reqwest::Client,
}

/// 所有请求的trait
pub trait Request {
    fn get_type() -> String;

    fn get_response_name() -> String;
}

impl Client {
    /// 根据[`Config`]创建[`Client`]
    pub fn new(config: Config) -> Self {
        Client {
            config,
            reqwest: reqwest::Client::new(),
        }
    }
    /// 从环境变量创建[`Client`]，参考[`Config::from_env()`]
    pub fn from_env() -> Result<Self, Error> {
        Ok(Client {
            config: Config::from_env()?,
            reqwest: reqwest::Client::new(),
        })
    }
    /// 文件上传
    ///
    /// Examples
    /// ```
    /// let client = Client::from_env()?;
    /// let req = PddGoodsFilespaceImageUpload {
    /// file: Some(PddFile::from_file("examples/1.jpg").await?),
    /// };
    /// let rsp = client.file_upload(req).await?;
    /// ```
    pub async fn file_upload<T>(self, req: T) -> Result<Value, Error>
    where
        T: Request + FileUploadRequest + Serialize,
    {
        let pub_par = init_public_parameters(
            self.config.access_token,
            self.config.client_id,
            T::get_type(),
        );

        let pub_parameters = serde_json::to_value(pub_par)?;

        let req_value = serde_json::to_value(&req)?;

        // 获取文件数据
        let Some((name, data)) = req.get_file() else {
            return Err(Error::FileNotFoundError);
        };

        // 添加签名
        let body = add_sign(&self.config.client_secret, pub_parameters, req_value);

        let file_part = Part::bytes(data).file_name(name);

        //获取body的参数
        // TODO:unwrap
        let bodya = body.as_object().unwrap();

        let mut url = String::new();

        for i in bodya {
            let value = match i.1 {
                Value::String(s) => s.as_str().to_string(),
                Value::Null => continue,
                _ => i.1.to_string(),
            };
            url.push_str(&format!("{}={}&", i.0, value));
        }

        let form = Form::new().part("file", file_part);

        trace!("request body:{:?}", form);

        // 拼接请求url
        let request_url = format!("{}?{}", self.config.upload_url, url);
        trace!("{}", request_url);

        // 使用reqwest请求from-data
        let rsp = self
            .reqwest
            .post(&request_url)
            .multipart(form)
            .send()
            .await?;
        let result = rsp.text().await?;

        trace!("response body:{:?}", result);
        Ok(serde_json::from_str(&result)?)
    }

    /// 发送请求
    pub async fn send<T: Request + Serialize>(self, req: T) -> Result<Value, Error> {
        let pub_par = init_public_parameters(
            self.config.access_token,
            self.config.client_id,
            T::get_type(),
        );
        let pub_parameters = serde_json::to_value(pub_par)?;

        let req_value = serde_json::to_value(req)?;

        let body = add_sign(&self.config.client_secret, pub_parameters, req_value);

        trace!("request body:{:?}", body);

        let rsp = self
            .reqwest
            .post(self.config.url)
            .header(CONTENT_TYPE, "application/json;charset=utf-8")
            .json(&body)
            .send()
            .await?;
        let result = rsp.json::<Value>().await?;

        trace!("response body:{:?}", result);
        Ok(result)
    }
}

fn init_public_parameters(
    access_token: Option<String>,
    client_id: String,
    type_name: String,
) -> PublicParameters {
    PublicParameters {
        access_token,
        client_id,
        data_type: Some("JSON".to_string()),
        timestamp: OffsetDateTime::now_utc().unix_timestamp(),
        type_: type_name,
        version: None,
    }
}

fn add_sign(client_secret: &str, mut pub_parameters: Value, mut parameters: Value) -> Value {
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
    #[error("没有文件")]
    FileNotFoundError,
    #[error("使用环境变量初始化Client失败")]
    EnvVarNotFoundError(#[from] env::VarError),
    #[error("Json解析错误")]
    JsonParsingFailure(#[from] serde_json::Error),
    #[error("文件读取错误")]
    FileIOFailure(#[from] std::io::Error),
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
        client.send(req).await.unwrap();
    }

    #[test]
    fn build_sign_test() {}
}
