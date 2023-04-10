
use chrono::{Local, Timelike};

use crate::types::{END_MA2_INDEX};

pub struct Ohlcs {
    closes: Vec<f64>,
    min: u32,
}

impl Ohlcs {
    pub fn new() -> Self {
        Self {
            closes: vec![],
            min: 999,
        }
    }

    pub fn get_ma(&self, period: u8) -> anyhow::Result<f64> {
        if self.closes.len() < period as usize {
            return Err(anyhow::anyhow!("closes.len() < period"));
        }
        let start_index = self.closes.len() - period as usize;
        let ma =  self.closes[start_index..].iter().fold(0.0, |acc, x| acc + x) / period as f64;
        Ok(ma)
    }

    pub async fn add_close(&mut self, price: f64) {
        let min = Local::now().minute();
        if self.min != Local::now().minute() {
            if self.closes.len() == END_MA2_INDEX {
                self.closes.remove(0);
            }
            self.closes.push(price);
            self.min = min;
            println!("{:?}", self.closes);
        }
    }
}
