//! Structured logging to stderr, in stable formats so existing log
//! pipelines keep working.
//!
//! * [`Format::Std`] — plain lines, `2006/01/02 15:04:05 msg`, with
//!   structured records as `LEVEL msg k=v ...`; used until a format is
//!   configured.
//! * [`Format::Json`] — one JSON object per record.
//! * [`Format::Text`] — logfmt-style `key=value` records.
//!
//! Logs are never written to stdout, which carries the stdio JSON-RPC stream.

use std::io::Write;
use std::sync::OnceLock;
use std::time::Duration;

/// Output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Plain timestamped lines.
    Std,
    /// JSON records.
    Json,
    /// Text records (logfmt).
    Text,
}

/// Severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Debug.
    Debug = -4,
    /// Info.
    Info = 0,
    /// Warn.
    Warn = 4,
    /// Error.
    Error = 8,
}

impl Level {
    fn name(self) -> &'static str {
        match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }

    /// Parses `debug|info|warn|warning|error` (case-insensitive; empty is
    /// info).
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_ascii_lowercase().as_str() {
            "debug" => Ok(Level::Debug),
            "info" | "" => Ok(Level::Info),
            "warn" | "warning" => Ok(Level::Warn),
            "error" => Ok(Level::Error),
            _ => Err(format!("unknown log level: {s}")),
        }
    }
}

/// Parses `json|text` (case-insensitive; empty is json).
pub fn parse_format(s: &str) -> Result<Format, String> {
    match s.to_ascii_lowercase().as_str() {
        "json" | "" => Ok(Format::Json),
        "text" => Ok(Format::Text),
        _ => Err(format!("unknown log format: {s}")),
    }
}

/// An attribute value.
#[derive(Debug, Clone)]
pub enum Val {
    /// A string.
    Str(String),
    /// An integer.
    Int(i64),
    /// A duration.
    Dur(Duration),
}

impl From<&str> for Val {
    fn from(s: &str) -> Self {
        Val::Str(s.to_string())
    }
}
impl From<String> for Val {
    fn from(s: String) -> Self {
        Val::Str(s)
    }
}
impl From<i64> for Val {
    fn from(i: i64) -> Self {
        Val::Int(i)
    }
}
impl From<u16> for Val {
    fn from(i: u16) -> Self {
        Val::Int(i as i64)
    }
}
impl From<Duration> for Val {
    fn from(d: Duration) -> Self {
        Val::Dur(d)
    }
}

struct Config {
    format: Format,
    level: Level,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

/// Configures the global logger (first call wins).
pub fn init(format: Format, level: Level) {
    let _ = CONFIG.set(Config { format, level });
}

fn config() -> &'static Config {
    CONFIG.get_or_init(|| Config {
        format: Format::Std,
        level: Level::Info,
    })
}

fn now_local() -> time::OffsetDateTime {
    time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc())
}

/// Logs a plain message. Once a structured format is configured, the
/// message becomes an INFO record in that format.
pub fn printf(msg: impl AsRef<str>) {
    if config().format != Format::Std {
        log(Level::Info, msg.as_ref().trim_end_matches('\n'), &[]);
        return;
    }
    std_line(msg.as_ref());
}

fn std_line(msg: &str) {
    let t = now_local();
    let line = format!(
        "{:04}/{:02}/{:02} {:02}:{:02}:{:02} {}\n",
        t.year(),
        t.month() as u8,
        t.day(),
        t.hour(),
        t.minute(),
        t.second(),
        msg.trim_end_matches('\n')
    );
    let _ = std::io::stderr().write_all(line.as_bytes());
}

/// Emits a structured record.
pub fn log(level: Level, msg: &str, attrs: &[(&str, Val)]) {
    let cfg = config();
    if level < cfg.level {
        return;
    }
    match cfg.format {
        Format::Std => {
            let mut s = format!("{} {}", level.name(), msg);
            for (k, v) in attrs {
                s.push(' ');
                s.push_str(&text_attr(k, v));
            }
            std_line(&s);
        }
        Format::Text => {
            let mut s = format!(
                "time={} level={} msg={}",
                rfc3339_millis(),
                level.name(),
                text_value(msg)
            );
            for (k, v) in attrs {
                s.push(' ');
                s.push_str(&text_attr(k, v));
            }
            s.push('\n');
            let _ = std::io::stderr().write_all(s.as_bytes());
        }
        Format::Json => {
            let mut m = serde_json::Map::new();
            m.insert("time".into(), rfc3339_millis().into());
            m.insert("level".into(), level.name().into());
            m.insert("msg".into(), msg.into());
            for (k, v) in attrs {
                let jv = match v {
                    Val::Str(s) => serde_json::Value::String(s.clone()),
                    Val::Int(i) => (*i).into(),
                    Val::Dur(d) => (d.as_nanos() as i64).into(),
                };
                m.insert((*k).to_string(), jv);
            }
            let mut b = crate::wire::json::to_json_bytes(&serde_json::Value::Object(m));
            b.push(b'\n');
            let _ = std::io::stderr().write_all(&b);
        }
    }
}

/// Info-level record.
pub fn info(msg: &str, attrs: &[(&str, Val)]) {
    log(Level::Info, msg, attrs);
}

/// Warn-level record.
pub fn warn(msg: &str, attrs: &[(&str, Val)]) {
    log(Level::Warn, msg, attrs);
}

fn rfc3339_millis() -> String {
    let t = now_local();
    let off = t.offset();
    let z = if off.is_utc() {
        "Z".to_string()
    } else {
        let (h, m, _) = off.as_hms();
        format!(
            "{}{:02}:{:02}",
            if h < 0 || m < 0 { '-' } else { '+' },
            h.abs(),
            m.abs()
        )
    };
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}{}",
        t.year(),
        t.month() as u8,
        t.day(),
        t.hour(),
        t.minute(),
        t.second(),
        t.millisecond(),
        z
    )
}

fn text_value(s: &str) -> String {
    let needs_quote = s.is_empty()
        || s.chars()
            .any(|c| c == ' ' || c == '=' || c == '"' || !crate::wire::fmt::is_print(c));
    if needs_quote {
        crate::wire::fmt::quote(s)
    } else {
        s.to_string()
    }
}

fn text_attr(k: &str, v: &Val) -> String {
    let vs = match v {
        Val::Str(s) => text_value(s),
        Val::Int(i) => i.to_string(),
        Val::Dur(d) => duration_string(*d),
    };
    format!("{k}={vs}")
}

/// Formats a duration compactly: `0s`, `850ns`, `1.5µs`, `12ms`,
/// `2.5s`, `1m30s`, `1h0m0s`.
pub fn duration_string(d: Duration) -> String {
    let ns = d.as_nanos();
    if ns == 0 {
        return "0s".into();
    }
    if ns < 1_000 {
        return format!("{ns}ns");
    }
    let trim = |s: String| {
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            s
        }
    };
    if ns < 1_000_000 {
        return format!("{}µs", trim(format!("{}.{:03}", ns / 1_000, ns % 1_000)));
    }
    if ns < 1_000_000_000 {
        return format!(
            "{}ms",
            trim(format!("{}.{:06}", ns / 1_000_000, ns % 1_000_000))
        );
    }
    let total_secs = ns / 1_000_000_000;
    let frac = ns % 1_000_000_000;
    let (h, m, s) = (total_secs / 3600, (total_secs / 60) % 60, total_secs % 60);
    let secs = trim(format!("{s}.{frac:09}"));
    if h > 0 {
        format!("{h}h{m}m{secs}s")
    } else if m > 0 {
        format!("{m}m{secs}s")
    } else {
        format!("{secs}s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations() {
        assert_eq!(
            duration_string(Duration::from_nanos(1_234_567)),
            "1.234567ms"
        );
        assert_eq!(duration_string(Duration::from_micros(350)), "350µs");
        assert_eq!(duration_string(Duration::from_millis(2500)), "2.5s");
        assert_eq!(duration_string(Duration::from_secs(90)), "1m30s");
        assert_eq!(duration_string(Duration::ZERO), "0s");
    }
}
