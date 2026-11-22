//! A Go `time.Time` work-alike for timestamps that travel through JSON.
//!
//! Parsing follows Go's `Time.UnmarshalJSON` (strict RFC 3339, with Go's
//! exact error wording), encoding follows `time.RFC3339Nano`, and
//! [`GoTime::format`] understands the Go reference-time layout tokens.

use std::fmt;

/// The zone a parsed time carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Zone {
    /// UTC (`Z`, or the zero value).
    #[default]
    Utc,
    /// A fixed numeric offset in seconds east of UTC.
    Fixed(i32),
}

/// An instant plus the zone offset it was written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GoTime {
    /// Seconds since 0001-01-01T00:00:00Z (Go's internal epoch); this keeps
    /// the zero value representable.
    secs: i64,
    nsec: u32,
    zone: Zone,
}

/// Seconds between 0001-01-01 and 1970-01-01.
const UNIX_TO_INTERNAL: i64 = 62_135_596_800;

impl Default for GoTime {
    /// Go's zero time, `0001-01-01T00:00:00Z`.
    fn default() -> Self {
        Self {
            secs: 0,
            nsec: 0,
            zone: Zone::Utc,
        }
    }
}

/// Civil date-time fields.
#[derive(Debug, Clone, Copy)]
struct Civil {
    year: i64,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
}

fn is_leap(y: i64) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

fn days_in(month: u32, year: i64) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(year) => 29,
        2 => 28,
        _ => 0,
    }
}

/// Days since 1970-01-01 for a proleptic Gregorian date.
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let m = m as i64;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const DAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

/// Appends one formatted layout element.
type FormatFn<'a> = dyn Fn(&mut String) + 'a;

const RFC3339: &str = "2006-01-02T15:04:05Z07:00";

/// Go's `ParseError.Error()`.
fn parse_error(value: &str, layout_elem: &str, value_elem: &str, message: &str) -> String {
    if !message.is_empty() {
        return format!("parsing time {}{message}", go_quote(value));
    }
    format!(
        "parsing time {} as {}: cannot parse {} as {}",
        go_quote(value),
        go_quote(RFC3339),
        go_quote(value_elem),
        go_quote(layout_elem)
    )
}

fn go_quote(s: &str) -> String {
    crate::go::fmt::quote(s)
}

fn digits(s: &[u8], n: usize) -> Option<u32> {
    if s.len() < n || !s[..n].iter().all(u8::is_ascii_digit) {
        return None;
    }
    Some(s[..n].iter().fold(0, |a, b| a * 10 + (b - b'0') as u32))
}

/// Go's `parseNanoseconds` for a `.ddd` prefix of length `n`.
fn parse_nanos(frac: &[u8]) -> u32 {
    // frac includes the leading separator.
    let ds = &frac[1..];
    let mut ns: u32 = 0;
    for i in 0..9 {
        ns *= 10;
        if i < ds.len() {
            ns += (ds[i] - b'0') as u32;
        }
    }
    ns
}

impl GoTime {
    fn from_civil(c: Civil, nsec: u32, zone: Zone) -> Self {
        let days = days_from_civil(c.year, c.month, c.day);
        let local = days * 86_400 + (c.hour * 3600 + c.min * 60 + c.sec) as i64;
        let off = match zone {
            Zone::Utc => 0,
            Zone::Fixed(o) => o as i64,
        };
        Self {
            secs: local - off + UNIX_TO_INTERNAL,
            nsec,
            zone,
        }
    }

    /// Constructs a UTC time from Unix seconds and nanoseconds.
    pub fn from_unix(secs: i64, nsec: u32) -> Self {
        Self {
            secs: secs + UNIX_TO_INTERNAL,
            nsec,
            zone: Zone::Utc,
        }
    }

    /// Unix seconds.
    pub fn unix(&self) -> i64 {
        self.secs - UNIX_TO_INTERNAL
    }

    /// Reports whether this is Go's zero time.
    pub fn is_zero(&self) -> bool {
        self.secs == 0 && self.nsec == 0
    }

    fn offset(&self) -> i32 {
        match self.zone {
            Zone::Utc => 0,
            Zone::Fixed(o) => o,
        }
    }

    fn civil(&self) -> (Civil, u32) {
        let local = self.secs - UNIX_TO_INTERNAL + self.offset() as i64;
        let days = local.div_euclid(86_400);
        let rem = local.rem_euclid(86_400) as u32;
        let (year, month, day) = civil_from_days(days);
        (
            Civil {
                year,
                month,
                day,
                hour: rem / 3600,
                min: rem / 60 % 60,
                sec: rem % 60,
            },
            (days + 4).rem_euclid(7) as u32,
        )
    }

    /// Parses like Go's `Time.UnmarshalJSON` (strict RFC 3339).
    pub fn parse_rfc3339(s: &str) -> Result<Self, String> {
        if let Some(t) = Self::parse_fast(s.as_bytes()) {
            return Ok(t);
        }
        // Fall back to the general parser for Go's error wording, then
        // apply the strictness checks Go layers on top of it.
        Self::parse_layout(s)?;
        let b = s.as_bytes();
        let num2 =
            |x: &[u8]| (x[0].wrapping_sub(b'0')) as u32 * 10 + (x[1].wrapping_sub(b'0')) as u32;
        if b.len() > 12 && b[12] == b':' {
            return Err(format!(
                "parsing time {} as {}: cannot parse {} as {}",
                go_quote(s),
                go_quote(RFC3339),
                go_quote(&s[11..12]),
                go_quote("15")
            ));
        }
        if b.len() > 19 && b[19] == b',' {
            return Err(format!(
                "parsing time {} as {}: cannot parse {} as {}",
                go_quote(s),
                go_quote(RFC3339),
                go_quote(","),
                go_quote(".")
            ));
        }
        if b.last() != Some(&b'Z') && b.len() >= 6 {
            if num2(&b[b.len() - 5..]) >= 24 {
                return Err(format!(
                    "parsing time {}: timezone hour out of range",
                    go_quote(s)
                ));
            }
            if num2(&b[b.len() - 2..]) >= 60 {
                return Err(format!(
                    "parsing time {}: timezone minute out of range",
                    go_quote(s)
                ));
            }
        }
        Err(format!(
            "parsing time {} as {}: cannot parse {} as {}",
            go_quote(s),
            go_quote(RFC3339),
            go_quote(s),
            go_quote(RFC3339)
        ))
    }

    /// Go's `parseRFC3339` fast path.
    fn parse_fast(s: &[u8]) -> Option<Self> {
        if s.len() < 19
            || s[4] != b'-'
            || s[7] != b'-'
            || s[10] != b'T'
            || s[13] != b':'
            || s[16] != b':'
        {
            return None;
        }
        let year = digits(s, 4)? as i64;
        let month = digits(&s[5..], 2)?;
        let day = digits(&s[8..], 2)?;
        let hour = digits(&s[11..], 2)?;
        let min = digits(&s[14..], 2)?;
        let sec = digits(&s[17..], 2)?;
        if !(1..=12).contains(&month)
            || day < 1
            || day > days_in(month, year)
            || hour > 23
            || min > 59
            || sec > 59
        {
            return None;
        }
        let mut rest = &s[19..];
        let mut nsec = 0;
        if rest.len() >= 2 && rest[0] == b'.' && rest[1].is_ascii_digit() {
            let mut n = 2;
            while n < rest.len() && rest[n].is_ascii_digit() {
                n += 1;
            }
            nsec = parse_nanos(&rest[..n]);
            rest = &rest[n..];
        }
        let civil = Civil {
            year,
            month,
            day,
            hour,
            min,
            sec,
        };
        match rest {
            [b'Z'] => Some(Self::from_civil(civil, nsec, Zone::Utc)),
            [sign @ (b'+' | b'-'), h1, h2, b':', m1, m2] => {
                let hh = digits(&[*h1, *h2], 2)?;
                let mm = digits(&[*m1, *m2], 2)?;
                if hh > 23 || mm > 59 {
                    return None;
                }
                let mut off = (hh * 3600 + mm * 60) as i32;
                if *sign == b'-' {
                    off = -off;
                }
                // An offset of zero lands in Local (UTC in our model) and
                // formats identically to Z.
                let zone = if off == 0 {
                    Zone::Utc
                } else {
                    Zone::Fixed(off)
                };
                Some(Self::from_civil(civil, nsec, zone))
            }
            _ => None,
        }
    }

    /// Go's general `time.Parse(time.RFC3339, s)`, used for error wording.
    fn parse_layout(value: &str) -> Result<Self, String> {
        let mut rest = value;
        let mut range_err = "";
        macro_rules! lit {
            ($p:literal) => {
                if let Some(r) = rest.strip_prefix($p) {
                    rest = r;
                } else {
                    return Err(parse_error(value, $p, rest, ""));
                }
            };
        }
        macro_rules! num {
            ($elem:literal, $fixed:expr) => {{
                let b = rest.as_bytes();
                if b.is_empty() || !b[0].is_ascii_digit() {
                    return Err(parse_error(value, $elem, rest, ""));
                }
                if b.len() < 2 || !b[1].is_ascii_digit() {
                    if $fixed {
                        return Err(parse_error(value, $elem, rest, ""));
                    }
                    let n = (b[0] - b'0') as u32;
                    rest = &rest[1..];
                    n
                } else {
                    let n = ((b[0] - b'0') * 10 + (b[1] - b'0')) as u32;
                    rest = &rest[2..];
                    n
                }
            }};
        }
        // 2006
        let b = rest.as_bytes();
        if b.len() < 4 || !b[..4].iter().all(u8::is_ascii_digit) {
            return Err(parse_error(value, "2006", rest, ""));
        }
        let year: i64 = rest[..4].parse().unwrap_or(0);
        rest = &rest[4..];
        lit!("-");
        let month = num!("01", true);
        if month == 0 || month > 12 {
            range_err = "month";
        }
        if !range_err.is_empty() {
            return Err(parse_error(
                value,
                "01",
                rest,
                &format!(": {range_err} out of range"),
            ));
        }
        lit!("-");
        let day = num!("02", true);
        if day > 31 {
            return Err(parse_error(value, "02", rest, ": day out of range"));
        }
        lit!("T");
        let hour = num!("15", false);
        if hour > 23 {
            return Err(parse_error(value, "15", rest, ": hour out of range"));
        }
        lit!(":");
        let min = num!("04", true);
        if min > 59 {
            return Err(parse_error(value, "04", rest, ": minute out of range"));
        }
        lit!(":");
        let sec = num!("05", true);
        if sec > 59 {
            return Err(parse_error(value, "05", rest, ": second out of range"));
        }
        let mut nsec = 0;
        let rb = rest.as_bytes();
        if rb.len() >= 2 && (rb[0] == b'.' || rb[0] == b',') && rb[1].is_ascii_digit() {
            let mut n = 2;
            while n < rb.len() && rb[n].is_ascii_digit() {
                n += 1;
            }
            nsec = parse_nanos(&rb[..n]);
            rest = &rest[n..];
        }
        // Z07:00
        let zone;
        if let Some(r) = rest.strip_prefix('Z') {
            rest = r;
            zone = Zone::Utc;
        } else {
            let hold = rest;
            let b = rest.as_bytes();
            if b.len() < 6 || b[3] != b':' {
                return Err(parse_error(value, "Z07:00", hold, ""));
            }
            let (Some(hr), Some(mm)) = (digits(&b[1..3], 2), digits(&b[4..6], 2)) else {
                return Err(parse_error(value, "Z07:00", hold, ""));
            };
            rest = &rest[6..];
            if hr > 24 {
                return Err(parse_error(
                    value,
                    "Z07:00",
                    rest,
                    ": time zone offset hour out of range",
                ));
            }
            if mm > 60 {
                return Err(parse_error(
                    value,
                    "Z07:00",
                    rest,
                    ": time zone offset minute out of range",
                ));
            }
            let mut off = (hr * 3600 + mm * 60) as i32;
            match b[0] {
                b'+' => {}
                b'-' => off = -off,
                _ => return Err(parse_error(value, "Z07:00", hold, "")),
            }
            zone = if off == 0 {
                Zone::Utc
            } else {
                Zone::Fixed(off)
            };
        }
        if !rest.is_empty() {
            return Err(format!(
                "parsing time {}: extra text: {}",
                go_quote(value),
                go_quote(rest)
            ));
        }
        if day < 1 || day > days_in(month, year) {
            return Err(format!(
                "parsing time {}: day out of range",
                go_quote(value)
            ));
        }
        Ok(Self::from_civil(
            Civil {
                year,
                month,
                day,
                hour,
                min,
                sec,
            },
            nsec,
            zone,
        ))
    }

    /// Formats as Go's `time.RFC3339Nano` (trailing zeros trimmed, `Z`
    /// for a zero offset).
    pub fn format_rfc3339_nano(&self) -> String {
        self.format("2006-01-02T15:04:05.999999999Z07:00")
    }

    /// Formats with a Go reference-time layout.
    pub fn format(&self, layout: &str) -> String {
        let (c, wd) = self.civil();
        let off = self.offset();
        let mut out = String::new();
        let mut l = layout;
        while !l.is_empty() {
            let tokens: &[(&str, &FormatFn)] = &[
                ("January", &|o: &mut String| {
                    o.push_str(MONTHS[c.month as usize - 1])
                }),
                ("Jan", &|o: &mut String| {
                    o.push_str(&MONTHS[c.month as usize - 1][..3])
                }),
                ("Monday", &|o: &mut String| o.push_str(DAYS[wd as usize])),
                ("Mon", &|o: &mut String| o.push_str(&DAYS[wd as usize][..3])),
                ("MST", &|o: &mut String| {
                    if self.zone == Zone::Utc {
                        o.push_str("UTC")
                    } else {
                        o.push_str(&fmt_offset(off, false, false))
                    }
                }),
                ("2006", &|o: &mut String| {
                    o.push_str(&format!("{:04}", c.year))
                }),
                ("Z07:00", &|o: &mut String| {
                    if off == 0 {
                        o.push('Z')
                    } else {
                        o.push_str(&fmt_offset(off, true, false))
                    }
                }),
                ("Z0700", &|o: &mut String| {
                    if off == 0 {
                        o.push('Z')
                    } else {
                        o.push_str(&fmt_offset(off, false, false))
                    }
                }),
                ("-07:00", &|o: &mut String| {
                    o.push_str(&fmt_offset(off, true, false))
                }),
                ("-0700", &|o: &mut String| {
                    o.push_str(&fmt_offset(off, false, false))
                }),
                ("-07", &|o: &mut String| {
                    o.push_str(&fmt_offset(off, false, true))
                }),
                ("01", &|o: &mut String| {
                    o.push_str(&format!("{:02}", c.month))
                }),
                ("02", &|o: &mut String| o.push_str(&format!("{:02}", c.day))),
                ("06", &|o: &mut String| {
                    o.push_str(&format!("{:02}", c.year.rem_euclid(100)))
                }),
                ("15", &|o: &mut String| {
                    o.push_str(&format!("{:02}", c.hour))
                }),
                ("03", &|o: &mut String| {
                    o.push_str(&format!("{:02}", h12(c.hour)))
                }),
                ("04", &|o: &mut String| o.push_str(&format!("{:02}", c.min))),
                ("05", &|o: &mut String| o.push_str(&format!("{:02}", c.sec))),
                ("PM", &|o: &mut String| {
                    o.push_str(if c.hour >= 12 { "PM" } else { "AM" })
                }),
                ("pm", &|o: &mut String| {
                    o.push_str(if c.hour >= 12 { "pm" } else { "am" })
                }),
            ];
            // Fractional seconds: .000 / .999 (or , separator).
            if (l.starts_with(".0")
                || l.starts_with(".9")
                || l.starts_with(",0")
                || l.starts_with(",9"))
                && !out.is_empty()
            {
                let ch = l.as_bytes()[1];
                let n = l[1..].bytes().take_while(|b| *b == ch).count();
                let after = l.as_bytes().get(1 + n);
                if after.is_none_or(|b| !b.is_ascii_digit()) {
                    let sep = &l[..1];
                    let digits = format!("{:09}", self.nsec);
                    let mut d = digits[..n.min(9)].to_string();
                    if ch == b'9' {
                        let trimmed = d.trim_end_matches('0').to_string();
                        if !trimmed.is_empty() {
                            out.push_str(sep);
                            out.push_str(&trimmed);
                        }
                    } else {
                        while d.len() < n {
                            d.push('0');
                        }
                        out.push_str(sep);
                        out.push_str(&d);
                    }
                    l = &l[1 + n..];
                    continue;
                }
            }
            if let Some((tok, f)) = tokens.iter().find(|(t, _)| l.starts_with(t)) {
                f(&mut out);
                l = &l[tok.len()..];
                continue;
            }
            // Single-digit tokens.
            let single: Option<String> = match l.as_bytes()[0] {
                b'1' => Some(c.month.to_string()),
                b'2' => Some(c.day.to_string()),
                b'3' => Some(h12(c.hour).to_string()),
                b'4' => Some(c.min.to_string()),
                b'5' => Some(c.sec.to_string()),
                _ => None,
            };
            if let Some(s) = single {
                out.push_str(&s);
                l = &l[1..];
                continue;
            }
            let ch = l.chars().next().unwrap_or(' ');
            out.push(ch);
            l = &l[ch.len_utf8()..];
        }
        out
    }
}

fn h12(h: u32) -> u32 {
    match h % 12 {
        0 => 12,
        x => x,
    }
}

fn fmt_offset(off: i32, colon: bool, hours_only: bool) -> String {
    let sign = if off < 0 { '-' } else { '+' };
    let a = off.unsigned_abs();
    let (h, m) = (a / 3600, a / 60 % 60);
    if hours_only {
        format!("{sign}{h:02}")
    } else if colon {
        format!("{sign}{h:02}:{m:02}")
    } else {
        format!("{sign}{h:02}{m:02}")
    }
}

/// Go's `Time.String()`: `2006-01-02 15:04:05.999999999 -0700 MST`.
impl fmt::Display for GoTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format("2006-01-02 15:04:05.999999999 -0700 MST"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc3339_roundtrip_and_format() {
        let cases = [
            (
                "2024-03-05T10:20:30.123-07:00",
                "2024-03-05T10:20:30.123-07:00",
                "2024-03-05 10:20",
                "2024-03-05 10:20:30.123 -0700 -0700",
            ),
            (
                "2024-03-05T10:20:30Z",
                "2024-03-05T10:20:30Z",
                "2024-03-05 10:20",
                "2024-03-05 10:20:30 +0000 UTC",
            ),
            (
                "2024-03-05T10:20:30+00:00",
                "2024-03-05T10:20:30Z",
                "2024-03-05 10:20",
                "2024-03-05 10:20:30 +0000 UTC",
            ),
            (
                "2024-03-05T10:20:30.120000+05:30",
                "2024-03-05T10:20:30.12+05:30",
                "2024-03-05 10:20",
                "2024-03-05 10:20:30.12 +0530 +0530",
            ),
        ];
        for (input, json, short, string) in cases {
            let t = GoTime::parse_rfc3339(input).unwrap();
            assert_eq!(t.format_rfc3339_nano(), json);
            assert_eq!(t.format("2006-01-02 15:04"), short);
            assert_eq!(t.to_string(), string);
        }
        assert_eq!(
            GoTime::default().format_rfc3339_nano(),
            "0001-01-01T00:00:00Z"
        );
    }

    #[test]
    fn rfc3339_errors() {
        let cases = [
            (
                "bad",
                r#"parsing time "bad" as "2006-01-02T15:04:05Z07:00": cannot parse "bad" as "2006""#,
            ),
            (
                "2024-01-01",
                r#"parsing time "2024-01-01" as "2006-01-02T15:04:05Z07:00": cannot parse "" as "T""#,
            ),
            (
                "2024-01-01T10:00:00+0000",
                r#"parsing time "2024-01-01T10:00:00+0000" as "2006-01-02T15:04:05Z07:00": cannot parse "+0000" as "Z07:00""#,
            ),
            (
                "2024-13-01T10:00:00Z",
                r#"parsing time "2024-13-01T10:00:00Z": month out of range"#,
            ),
            (
                "2024-02-30T10:00:00Z",
                r#"parsing time "2024-02-30T10:00:00Z": day out of range"#,
            ),
        ];
        for (input, want) in cases {
            assert_eq!(GoTime::parse_rfc3339(input).unwrap_err(), want, "{input}");
        }
    }
}
