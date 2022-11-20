use std::time::SystemTime;

#[inline]
pub fn default_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[inline]
pub fn relative_since(secs: u64) -> String {
    get_relative_time(default_time() - secs)
}

#[inline]
pub fn get_relative_time(time: u64) -> String {
    format!("<t:{time}:R>")
}
