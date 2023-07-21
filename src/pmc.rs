use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use base64::{engine::general_purpose, Engine};
use futures_util::{SinkExt, Stream, StreamExt};
use md5::{Digest, Md5};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::trace;

use crate::{Config, Error};

pub struct PmcClient {
    pub config: Config,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PmcMessage {
    id: Option<i128>,
    #[serde(rename = "commandType")]
    command_type: CommandType,
    time: u128,
    message: Option<Message>,
    #[serde(rename = "sendTime")]
    send_time: Option<u128>,
}

impl From<tokio_tungstenite::tungstenite::Message> for PmcMessage {
    fn from(value: tokio_tungstenite::tungstenite::Message) -> Self {
        let tokio_tungstenite::tungstenite::Message::Text(t) = value else{
            panic!("e");
        };
        serde_json::from_str(&t).unwrap()
    }
}

impl PmcMessage {
    fn to_ack(&self) -> tokio_tungstenite::tungstenite::Message {
        let ack = format!(
            r##"{{"commandType":"Ack","time":{},"id":{},"sendTime":{},"type":"{}","mallID":{}}}"##,
            unix_timestamp_millis(),
            self.id.unwrap(),
            self.send_time.unwrap(),
            self.message.as_ref().unwrap().msg_type,
            self.message.as_ref().unwrap().mall_id
        );
        tokio_tungstenite::tungstenite::Message::text(ack)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "mallID")]
    mall_id: i128,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum CommandType {
    HeartBeat,
    Ack,
    Fail,
    Common,
}

pub struct PmcConsumers(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl PmcConsumers {
    pub async fn ack(&mut self, msg: &PmcMessage) {
        let ack_msg = msg.to_ack();
        trace!("ack:{:?}", ack_msg);
        self.0.send(ack_msg).await.unwrap();
    }
}

impl Stream for PmcConsumers {
    type Item = Result<PmcMessage, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.0.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(m)) => match m {
                Ok(msg) => {
                    let msg_value: PmcMessage = msg.into();
                    Poll::Ready(Some(Ok(msg_value)))
                }
                Err(_) => Poll::Ready(Some(Err(Error::PmcFailure))),
            },
        }
    }
}

impl PmcClient {
    /// 创建 [`PmcClient`].
    pub fn new(config: Config) -> Self {
        PmcClient { config }
    }

    pub async fn connect(&self) -> Result<PmcConsumers, Error> {
        let time = unix_timestamp_millis();
        let sign = sign(&self.config.client_id, time, &self.config.client_secret);

        let wss_path = format!(
            "wss://message-api.pinduoduo.com/message/{}/{}/{}",
            self.config.client_id, time, sign
        );
        if let Ok(s) = connect_async(Url::parse(&wss_path).unwrap()).await {
            Ok(PmcConsumers(s.0))
        } else {
            Err(Error::PmcConnectionFailure)
        }
    }
}

fn unix_timestamp_millis() -> u128 {
    Duration::from_nanos(time::OffsetDateTime::now_utc().unix_timestamp_nanos() as u64).as_millis()
}

fn sign(client_id: &str, time: u128, client_secret: &str) -> String {
    let mut data = String::new();
    data.push_str(client_id);
    data.push_str(&time.to_string());
    data.push_str(client_secret);
    let mut h = Md5::new();
    h.update(data);
    let md5 = format!("{:x}", h.finalize());
    general_purpose::STANDARD.encode(md5)
}
