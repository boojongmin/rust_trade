
use chrono::{Local, Timelike};
use db::DB;

use crate::types::{END_MA2_INDEX};

use crate::types::AR;
use crate::util::{get_datetime_str_from_datetime_what_second_is_zero};

pub struct Ohlcs {
    closes: Vec<f64>,
    min: u32,
    db: AR<DB>,
}

impl Ohlcs {
    pub async fn new(db: AR<DB>) -> Self {
        let mut closes = vec![];
        {
            let db  = db.read().await;
            let candles = db::candle_select_limit_120(&db).await;
            closes = candles.iter().map(|c| c.get_close()).collect()
        }

        Self {
            db,
            closes,
            min: 999,
        }
    }

    pub fn get_ma(&self, period: usize) -> anyhow::Result<f64> {
        if self.closes.len() < period as usize {
            return Err(anyhow::anyhow!("closes.len() < period"));
        }
        let start_index = self.closes.len() - period;
        let ma =  self.closes[start_index..].iter().fold(0.0, |acc, x| acc + x) / period as f64;
        Ok(ma)
    }

    pub async fn add_close_price_and_insert_db(&mut self, price: f64) {
        let now = Local::now();
        let min = now.minute();
        if self.min != now.minute() {
            if self.closes.len() == END_MA2_INDEX {
                self.closes.remove(0);
            }
            self.closes.push(price);
            self.min = min;
            println!("{:?}", self.closes);

            // 직전 분봉 정보 db insert
            let i = self.closes.len() -1;
            let db = self.db.read().await;
            let min_string = get_datetime_str_from_datetime_what_second_is_zero(&now);
            db::candle_insert(&db, self.closes[i], min_string.as_str()).await;
        } else {
            let i = self.closes.len() -1;
            self.closes[i] = price;
        }
    }
}
