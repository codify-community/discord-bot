use core::fmt;
use std::time::{Duration, SystemTime};

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

// Credits: https://github.com/rust-lang/rustup/blob/b3d53252ec06635da4b8bd434a82e2e8b6480485/src/cli/download_tracker.rs#L262
pub fn format_dhms(sec: u64) -> (u64, u8, u8, u8) {
    let (mins, sec) = (sec / 60, (sec % 60) as u8);
    let (hours, mins) = (mins / 60, (mins % 60) as u8);
    let (days, hours) = (hours / 24, (hours % 24) as u8);

    (days, hours, mins, sec)
}

pub struct Humanize(pub Duration);

impl fmt::Display for Humanize {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const SECS_PER_YEAR: u64 = 60 * 60 * 24 * 365;

        let secs = self.0.as_secs();
        if secs > SECS_PER_YEAR {
            return f.write_str("Desconhecido");
        }

        match format_dhms(secs) {
            (0, 0, 0, s) => write!(f, "{s} segundos"),
            (0, 0, m, s) => write!(f, "{m} minutos e {s} seconds"),
            (0, h, m, s) => write!(f, "{h} horas, {m} minutos e {s} segundos"),
            (d, h, m, s) => write!(f, "{d} dias, {h} horas, {m} minutos e {s}s"),
        }
    }
}
