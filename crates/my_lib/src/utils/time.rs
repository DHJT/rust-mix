//! Timer implementation for the Sentinel.
//! It supports a cached timer and a real-time timer from `unix_timestamp_nanos`.

#[inline]
pub fn sleep_for_ms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
    println!("sleep_for_ms!");
}

#[inline]
pub fn sleep_for_ns(ns: u64) {
    std::thread::sleep(std::time::Duration::from_nanos(ns));
}
