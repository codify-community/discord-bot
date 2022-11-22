use std::time::SystemTime;

#[inline]
pub fn default() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[inline]
pub fn relative_since(secs: u64) -> String {
    discord_relative_format(default() - secs)
}

#[inline]
pub fn discord_relative_format(time: u64) -> String {
    format!("<t:{time}:R>")
}
