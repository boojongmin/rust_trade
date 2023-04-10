use bot::{ws::{Ws, spawn_rx_and_add_close_price}, candle::Ohlcs, util::{lock_new, lock_clone}};




#[tokio::main]
async fn main() {
    let ws = Ws::new().await;
    ws.start();

    let rx_lock = ws.get_rx_lock();
    let candle_lock = lock_new(Ohlcs::new());
    spawn_rx_and_add_close_price(rx_lock, candle_lock);

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

//create test function
#[test]
fn test() {
    
}
