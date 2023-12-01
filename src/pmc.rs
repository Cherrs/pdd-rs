use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use base64::{engine::general_purpose, Engine};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, Stream, StreamExt,
};
use md5::{Digest, Md5};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::{error, trace};

use crate::{Config, Error};

pub struct PmcClient {
    pub config: Config,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PmcMessage {
    pub id: Option<i128>,
    #[serde(rename = "commandType")]
    pub command_type: CommandType,
    pub time: u128,
    pub message: Option<Message>,
    #[serde(rename = "sendTime")]
    pub send_time: Option<u128>,
}

impl From<tokio_tungstenite::tungstenite::Message> for PmcMessage {
    fn from(value: tokio_tungstenite::tungstenite::Message) -> Self {
        let tokio_tungstenite::tungstenite::Message::Text(t) = value else {
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
pub struct Message {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(rename = "mallID")]
    pub mall_id: i128,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommandType {
    HeartBeat,
    Ack,
    Fail,
    Common,
}

pub struct PmcConsumers(
    SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    Arc<
        Mutex<
            SplitSink<
                WebSocketStream<MaybeTlsStream<TcpStream>>,
                tokio_tungstenite::tungstenite::Message,
            >,
        >,
    >,
);

impl PmcConsumers {
    pub async fn ack(&mut self, msg: &PmcMessage) {
        let ack_msg = msg.to_ack();
        trace!("ack:{:?}", ack_msg);
        self.1.lock().await.send(ack_msg).await.unwrap();
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
                    if let tokio_tungstenite::tungstenite::Message::Close(e) = msg {
                        error!("连接断开,错误：{:?}", e);
                        return Poll::Ready(Some(Err(Error::PmcConnectionFailure)));
                    }
                    let msg_value: PmcMessage = msg.into();
                    trace!("收到消息:{:?}", msg_value);
                    if let CommandType::HeartBeat = msg_value.command_type {
                        cx.waker().wake_by_ref();
                        return Poll::Pending;
                    }
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
            let (w, r) = s.0.split();

            let send = Arc::new(Mutex::new(w));
            let b_send = send.clone();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(5));
                loop {
                    let heat = format!(
                        r##"{{"commandType":"HeartBeat","time":{}}}"##,
                        unix_timestamp_millis()
                    );
                    trace!("发送心跳:{}", heat);
                    b_send
                        .lock()
                        .await
                        .send(tokio_tungstenite::tungstenite::Message::Text(heat))
                        .await
                        .unwrap();
                    interval.tick().await;
                }
            });
            Ok(PmcConsumers(r, send))
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
