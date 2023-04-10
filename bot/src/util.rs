use std::sync::Arc;

use chrono::{DateTime, Local};
use tokio::sync::RwLock;


pub fn lock_new<T>(t: T) -> Arc<RwLock<T>>
{
    Arc::new(RwLock::new(t))
}

pub fn lock_clone<T>(lock: &Arc<RwLock<T>>) -> Arc<RwLock<T>>
{
    Arc::clone(lock)
}


pub fn get_datetime_str_from_datetime_what_second_is_zero(native_datetime: &DateTime<Local>) -> String {
    native_datetime.format("%Y-%m-%d %H:%M:00").to_string()
}

pub fn is_golden_cross(ma1: f64, ma2: f64) -> bool {
    return ma1 > ma2
}