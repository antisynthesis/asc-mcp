//! Go `fmt`/`strconv` formatting helpers used to reproduce tool output.

/// Go `strconv.Quote` (what `%q` prints for a string).
pub fn quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\x07' => out.push_str("\\a"),
            '\x08' => out.push_str("\\b"),
            '\x0c' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\x0b' => out.push_str("\\v"),
            c if is_print(c) => out.push(c),
            c if (c as u32) < 0x20 || c as u32 == 0x7f => out.push_str(&format!("\\x{:02x}", c as u32)),
            c if (c as u32) < 0x10000 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push_str(&format!("\\U{:08x}", c as u32)),
        }
    }
    out.push('"');
    out
}

/// Approximates Go's `strconv.IsPrint`: graphic characters plus the
/// ASCII space; control, format, separator (other than U+0020) and
/// private-use characters are not printable.
pub fn is_print(c: char) -> bool {
    let u = c as u32;
    if c == ' ' {
        return true;
    }
    if c.is_control() || c.is_whitespace() {
        return false;
    }
    !matches!(u,
        0x00AD | 0x0600..=0x0605 | 0x061C | 0x06DD | 0x070F | 0x0890..=0x0891 | 0x08E2
        | 0x180E | 0x200B..=0x200F | 0x202A..=0x202E | 0x2060..=0x2064 | 0x2066..=0x206F
        | 0xFEFF | 0xFFF9..=0xFFFB | 0x110BD | 0x110CD | 0x13430..=0x1343F
        | 0x1BCA0..=0x1BCA3 | 0x1D173..=0x1D17A | 0xE0001 | 0xE0020..=0xE007F
        | 0xE000..=0xF8FF | 0xF0000..=0x10FFFF | 0xFFFE | 0xFFFF)
}

/// Go `%v` (and `%g`) for a float64.
pub fn float_v(f: f64) -> String {
    if f.is_nan() {
        return "NaN".into();
    }
    if f.is_infinite() {
        return if f > 0.0 { "+Inf".into() } else { "-Inf".into() };
    }
    if f == 0.0 {
        return if f.is_sign_negative() { "-0".into() } else { "0".into() };
    }
    let e = format!("{f:e}");
    let (mant, exp) = e.split_once('e').unwrap_or((&e, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    if !(-4..6).contains(&exp) {
        let sign = if exp < 0 { '-' } else { '+' };
        format!("{mant}e{sign}{:02}", exp.abs())
    } else {
        format!("{f}")
    }
}

/// Go `%.Nf` for a float64 (round half to even on the exact binary value,
/// which is what Rust's formatter also does).
pub fn float_prec(f: f64, prec: usize) -> String {
    format!("{f:.prec$}")
}

/// Go `%v` for a `[]string`: `[a b c]`.
pub fn slice_v<T: std::fmt::Display>(items: &[T]) -> String {
    let parts: Vec<String> = items.iter().map(|i| i.to_string()).collect();
    format!("[{}]", parts.join(" "))
}

/// Go `%v` for a bool.
pub fn bool_v(b: bool) -> &'static str {
    if b { "true" } else { "false" }
}

/// Go `%v` for a pointer that is printed only when nil (`<nil>`).
pub const NIL: &str = "<nil>";

/// Go `strings.Title`-free helper: Go `%s` of an error is its message;
/// provided for symmetry when porting.
pub fn err_s<E: std::fmt::Display>(e: &E) -> String {
    e.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quote_matches_go() {
        assert_eq!(quote("a\"b\n\x01é☺\u{200b}"), "\"a\\\"b\\n\\x01é☺\\u200b\"");
    }

    #[test]
    fn float_v_matches_go() {
        let cases = [
            (0.0, "0"), (1.0, "1"), (1.5, "1.5"), (100.0, "100"), (1234567.0, "1.234567e+06"),
            (123456.7, "123456.7"), (1e21, "1e+21"), (1e20, "1e+20"), (0.0001, "0.0001"),
            (0.00001, "1e-05"), (4.99, "4.99"), (1.0 / 3.0, "0.3333333333333333"),
            (-2.5e-8, "-2.5e-08"), (100000000.0, "1e+08"),
        ];
        for (f, want) in cases {
            assert_eq!(float_v(f), want, "{f}");
        }
    }
}
