use std::time::SystemTime;

#[inline]
pub fn default_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[inline]
pub fn relative_time(relative_to: u64) -> String {
    format!("<t:{}:R>", default_time() - relative_to)
}
