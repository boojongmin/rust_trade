use bot::{ws::{Ws, spawn_rx_and_add_close_price}, candle::Ohlcs, util::{lock_new, lock_clone, is_golden_cross}, types::{START_MA1_INDEX, START_MA2_INDEX, END_MA2_INDEX, END_MA1_INDEX}};




#[tokio::main]
async fn main() {
    let ws = Ws::new().await;
    ws.start();

    let db_lock = lock_new(db::create_db(5).await);
    let rx_lock = ws.get_rx_lock();
    let pool_clone1 = lock_clone(&db_lock);
    let pool_clone2 = lock_clone(&db_lock);

    let candle_lock1 = lock_new(Ohlcs::new(pool_clone1).await);
    let candle_lock2 = lock_new(Ohlcs::new(pool_clone2).await);

    let mut rx = spawn_rx_and_add_close_price(rx_lock, candle_lock1);

    let mut cache_is_hold: [Option<bool>; 200] = [None; 200] ;
    loop {
        if let Some(price) = rx.recv().await {
            for i in START_MA1_INDEX..END_MA1_INDEX+1 {
                if cache_is_hold[i] == None {
                    let db = &db_lock.read().await;
                    let r = db::account_select_last_one(db, i as i32).await;
                    match r {
                        Some(account) => {
                            cache_is_hold[i] = Some(account.is_buy_position());
                        },
                        None => {
                            dbg!("account_select_last_one is empty");
                            cache_is_hold[i] = Some(false);
                        }
                    }
                }

                if let Some(is_hold) = cache_is_hold[i] {
                    let db = &db_lock.read().await;
                    let r1 = candle_lock2.read().await.get_ma(i);
                    let r2 = candle_lock2.read().await.get_ma(i+10);
                    if let (Ok(ma1), Ok(ma2)) = (r1, r2) {
                        let is_golden_cross = is_golden_cross(ma1, ma2);
                        println!(">>> ma1: {}, ma2: {}", ma1, ma2);
                        // buy
                        if is_golden_cross && is_hold == false {
                            // 매수는 ma1과 ma2의 가격차이가  이내 일때만 동작
                            if( ma1 - ma2).abs() < 0.0001 {
                                db::account_trade(db, i as i32, price, true).await;
                            } else {
                                dbg!(">>> 매수 조건이 아님");
                            }
                            db::account_trade(db, i as i32, price, true).await;

                        // sell
                        } else if is_golden_cross == false && is_hold {
                            db::account_trade(db, i as i32, price, false).await;
                        }
                    }
                    
                }
            }

        }
    }
}

//create test function
#[test]
fn test() {
    
}
