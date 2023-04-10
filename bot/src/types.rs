use std::sync::Arc;

use tokio::sync::{RwLock, mpsc::{Sender, Receiver}};

pub type AR<T> = Arc<RwLock<T>>;
pub type ArTx = AR<Sender<String>>;
pub type ArRx = AR<Receiver<String>>;

pub const START_MA1_INDEX: usize = 10;
pub const START_MA2_INDEX: usize = 20;
pub const END_MA1_INDEX: usize = 110;
pub const END_MA2_INDEX: usize = 120;
// pub const END_MA1_INDEX: usize = 10;
// pub const END_MA2_INDEX: usize = 20;