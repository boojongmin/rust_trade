use std::{sync::Arc, panic};

use futures_util::{SinkExt, stream, StreamExt, lock};
use serde::{Serialize, Deserialize};
use tokio::sync::{RwLock, mpsc::{Sender, Receiver}};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message, connect_async};

use crate::{util::{lock_clone, lock_new}, candle::Ohlcs, types::{ArTx, ArRx, AR}};



type WsStream =  WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
type ArStream = AR<WsStream>;

pub struct Ws {
    ws_stream: ArStream,
    tx: ArTx,
    rx: ArRx,
}

impl Ws {
    pub async fn new() -> Self {
        let x = create_wsstream().await;
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);
        Self {
            ws_stream: lock_new(x),
            tx: lock_new(tx),
            rx: lock_new(rx),
        }
    }

    pub fn start(&self) {
        let wsstream_lock = lock_clone(&self.ws_stream);
        let tx_lock = lock_clone(&self.tx);
        tokio::spawn(async move {
            loop_ws_work(wsstream_lock, tx_lock).await;
        });
    }

    pub fn get_rx_lock(&self) -> ArRx {
        lock_clone(&self.rx)
    }
}



async fn create_wsstream() -> WsStream {
    let url = url::Url::parse("wss://wss.coinone.co.kr/ws?token_type=web").unwrap();
    let (mut ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
    let send_message = Message::Text(r#"{"event":"subscribe","channel":"market_price"}"#.to_string() + "\n");
    ws_stream.send(send_message.clone()).await.unwrap();
    ws_stream.send(send_message).await.unwrap();
    ws_stream
}

async fn loop_ws_work(lock: ArStream, lock_tx: ArTx) {
    loop {
        let mut ws_stream = lock.write().await;
        if let Some(message) = ws_stream.next().await {
            match message {
                Ok(m) => {
                    let m = m.into_text().unwrap();
                    let before_message = m.clone();
                    let tx = lock_tx.write().await;
                    match tx.send(m).await {
                        Ok(_) => {}
                        Err(v) => println!("{:?}, {:?}", before_message, v),
                    }
                }
                Err(m) => {
                    println!("{:?}", m);
                }
            }
        }
    }
}

pub fn spawn_rx_and_add_close_price(rx_lock: ArRx, candle_lock: AR<Ohlcs>) {
    tokio::spawn(async move {
        let mut rx = rx_lock.write().await;
        while let Some(msg) = rx.recv().await {
            match serde_json::from_str::<WsResponse<MarketPrice>>(msg.as_str()) {
                Ok(v) => {
                    if let Ok(v) = v.data.get(0).unwrap().price.parse::<f64>() {
                        let mut candle = candle_lock.write().await;
                        candle.add_close(v).await;
                    }
                }
                Err(e) => {
                    println!("error: {:?} / {}", e, msg.as_str());
                }
            }
        }
    });

}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsResponse<T> {
    channel: String,
    timestamp: i64,
    data: Vec<T>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketPrice {
    currency: String,
    price: String,
}