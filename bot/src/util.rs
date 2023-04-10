use std::sync::Arc;

use tokio::sync::RwLock;


pub fn lock_new<T>(t: T) -> Arc<RwLock<T>>
{
    Arc::new(RwLock::new(t))
}

pub fn lock_clone<T>(lock: &Arc<RwLock<T>>) -> Arc<RwLock<T>>
{
    Arc::clone(lock)
}
