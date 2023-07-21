use std::{sync::Arc, thread, time::Duration};

use base64::{engine::general_purpose, Engine};
use futures_util::{lock::Mutex, SinkExt, StreamExt};
use md5::{Digest, Md5};
use pdd::Config;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use tokio_tungstenite::{connect_async, tungstenite::Error};
use tracing::{error, trace, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config = Config::from_env()?;

    let time = unix_timestamp_millis();
    let sign = sign(&config.client_id, time, &config.client_secret);
    trace!("{}", sign);
    let wss_path = format!(
        "wss://message-api.pinduoduo.com/message/{}/{}/{}",
        config.client_id, time, sign
    );

    let socket = connect_async(Url::parse(&wss_path).unwrap()).await;
    let (socket, rsp) = match socket {
        Ok(s) => s,
        Err(e) => match e {
            Error::Http(b) => {
                error!("{}", String::from_utf8(b.body().clone().unwrap()).unwrap());
                panic!("e");
            }
            _ => panic!("e"),
        },
    };
    trace!("ws连接成功 {:?}", rsp);

    let (w, r) = socket.split();
    let ww = Arc::new(Mutex::new(w));
    let w = ww.clone();
    tokio::spawn(async move {
        loop {
            let heat = format!(
                r##"{{"commandType":"HeartBeat","time":{}}}"##,
                unix_timestamp_millis()
            );
            trace!("发送心跳:{}", heat);
            w.lock()
                .await
                .send(tokio_tungstenite::tungstenite::Message::Text(heat))
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
    r.for_each(|message| async {
        let w = ww.clone();
        let m: MessagePacket = message.unwrap().into();
        trace!("{:?}", m);
        match m.command_type {
            CommandType::HeartBeat => {}
            _ => {
                tokio::spawn(async move {
                    let t = thread::current().id();
                    trace!("{:?}", t);
                    let ack = m.to_ack();
                    w.lock().await.send(ack).await.unwrap();
                });
            }
        };
    })
    .await;
    // pin_mut!(ws_to_stdout, heat);
    // future::select(ws_to_stdout, heat).await;
    Ok(())
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MessagePacket {
    id: Option<i128>,
    #[serde(rename = "commandType")]
    command_type: CommandType,
    time: u128,
    message: Option<Message>,
    #[serde(rename = "sendTime")]
    send_time: Option<u128>,
}

impl From<tokio_tungstenite::tungstenite::Message> for MessagePacket {
    fn from(value: tokio_tungstenite::tungstenite::Message) -> Self {
        let tokio_tungstenite::tungstenite::Message::Text(t) = value else{
            panic!("e");
        };
        serde_json::from_str(&t).unwrap()
    }
}

impl MessagePacket {
    fn to_ack(&self) -> tokio_tungstenite::tungstenite::Message {
        let ack = format!(
            r##"{{"commandType":"Ack","time":{},"id":{},"sendTime":{},"type":"{}","mallID":{}}}"##,
            unix_timestamp_millis(),
            self.id.unwrap(),
            self.send_time.unwrap(),
            self.message.as_ref().unwrap().msg_type,
            self.message.as_ref().unwrap().mall_id
        );
        trace!("ack:{}", ack);
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

fn unix_timestamp_millis() -> u128 {
    Duration::from_nanos(time::OffsetDateTime::now_utc().unix_timestamp_nanos() as u64).as_millis()
}
