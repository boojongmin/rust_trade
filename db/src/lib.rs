use anyhow::Result;

use chrono::Utc;
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, PgConnection, Connection, types::chrono::{NaiveDateTime, DateTime, Local}};


pub struct DB {
    pool: PgPool,
}

pub async fn create_db(max: u32) -> DB {
    let pool = PgPoolOptions::new()
        .max_connections(max)
        .connect("postgres://postgres:postgres@localhost/postgres")
        .await
        .unwrap();
    DB { pool }
}


pub struct DbCandle {
    id: i32,
    close: f64,
    created_at: NaiveDateTime,
}

impl DbCandle {
    pub fn get_close(&self) -> f64 {
        self.close
    }
}

pub async fn candle_select_limit_120(db: &DB) -> Vec<DbCandle> {
    let r = sqlx::query_as!(DbCandle, r#"select * from candles order by created_at desc limit 120"#)
        .fetch_all(&db.pool)
        .await.unwrap();
    r
}

pub async fn candle_count(db: &DB) -> i64 {
    let r = sqlx::query!("select count(*) as count from candles")
        .fetch_one(&db.pool)
        .await
        .unwrap();
    r.count.unwrap()
}

pub async fn candle_insert(db: &DB, close_price: f64, create_at: &str) {
    let date_time = NaiveDateTime::parse_from_str(create_at, "%Y-%m-%d %H:%M:%S").unwrap();
    let r = sqlx::query!(r#"insert into candles(close, created_at) values($1, $2)"#, close_price, date_time).fetch_one(&db.pool)
        .await;
    match r {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}

#[derive(Debug)]
pub struct DBAccount {
    id: i32,
    strategy_id: i32,
    balance: f64,
    price: f64,
    quantity: f64,
    created_at: NaiveDateTime,
}

impl DBAccount {
    pub fn is_buy_position(&self) -> bool {
        if self.quantity > 0.0 {
            true
        } else {
            false
        }
    }
}
    
pub async fn account_select_last_one(db: &DB, strategy_id: i32) -> Option<DBAccount> {
    let r= sqlx::query_as!(DBAccount, r#"select * from account_history where strategy_id = $1 order by created_at desc limit 1"#, strategy_id) .fetch_one(&db.pool) .await;
    let r = match r {
        Ok(v) => Some(v),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    };
    r
}

pub async fn account_trade(db: &DB, strategy_id: i32, price: f64, is_buy: bool) {
    let now = Utc::now().naive_utc();

    if let Some(v) =  account_select_last_one(db, strategy_id).await {
        let price = calc_one_satoshi(price);
        let quantity = match is_buy {
            true => v.balance / price,
            false => 0.0,
        };
        let balance = match is_buy {
            true => v.balance - price * quantity,
            false => v.balance + price * quantity,
        };
        println!("balance > {}", balance);
        let r = sqlx::query!(r#"insert into account_history(strategy_id, balance, price, quantity, created_at) values($1, $2, $3, $4, $5)"#, strategy_id, balance, price, quantity, now).fetch_one(&db.pool)
            .await;
        match r {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        // 초기값으로 전략마다 1000만원씩 insert를 해두었기 때문에 나올수 없는 구조.
        dbg!(">>> 음...? 이게 왜 나오지?");
    }
}

pub fn calc_one_satoshi(bit_won: f64) -> f64 {
    return bit_won / 100000000.0
}




//     let r = sqlx::query!(r#"insert into account_history(strategy_id, balance, price, quantity, created_at) values($1, $2, $3, $4, $5)"#, strategy_id, 0.0, price, quantity, now).fetch_one(&db.pool)
//         .await;
//     match r {
//         Ok(_) => (),
//         Err(e) => println!("Error: {}", e),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    pub async fn delete_all(pool: &PgPool) {
        // let _ = sqlx::query!(r#"truncate table users"#).fetch_one(pool).await;
        // let _ = sqlx::query!(r#"truncate table candles"#).fetch_one(pool).await;
    }

    async fn init() -> DB {
        let db = create_db(1).await;
        delete_all(&db.pool).await;
        db
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_candle_count() {
        let p = &init().await;
        let count = candle_count(p).await;
        assert_eq!(count, 0);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_candle_insert() {
        let p = &init().await;
        let count1 = candle_count(p).await;
        candle_insert(&p, 10.0, "2023-04-10 19:10:00").await;
        let count2 = candle_count(p).await;
        assert_eq!(count1 + 1, count2);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_account_last_one() {
        let p = &init().await;
        let r = account_select_last_one(p, 10).await;
        println!("{:?}", r);
    }
}
