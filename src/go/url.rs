//! Go `net/url` escaping and `url.Values` encoding.

use std::collections::BTreeMap;

fn is_unreserved(c: u8) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_' | b'.' | b'~')
}

fn escape(s: &str, keep: impl Fn(u8) -> bool, space_plus: bool) -> String {
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        if keep(b) {
            out.push(b as char);
        } else if b == b' ' && space_plus {
            out.push('+');
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// Go `url.QueryEscape`.
pub fn query_escape(s: &str) -> String {
    escape(s, is_unreserved, true)
}

/// Go `url.PathEscape`.
pub fn path_escape(s: &str) -> String {
    escape(s, |c| is_unreserved(c) || matches!(c, b'$' | b'&' | b'+' | b':' | b'=' | b'@'), false)
}

/// Go `url.QueryUnescape`.
pub fn query_unescape(s: &str) -> Result<String, String> {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'%' => {
                let hex = s.get(i + 1..i + 3).filter(|h| h.bytes().all(|c| c.is_ascii_hexdigit()));
                match hex {
                    Some(h) => {
                        out.push(u8::from_str_radix(h, 16).unwrap_or(0));
                        i += 3;
                    }
                    None => {
                        let bad: String = s[i..].chars().take(3).collect();
                        return Err(format!("invalid URL escape {}", crate::go::fmt::quote(&bad)));
                    }
                }
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    Ok(String::from_utf8_lossy(&out).into_owned())
}

/// Go `url.Values`: an ordered multimap whose `encode` sorts by key.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Values(BTreeMap<String, Vec<String>>);

impl Values {
    /// An empty set of values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Go `Values.Set`.
    pub fn set(&mut self, k: impl Into<String>, v: impl Into<String>) {
        self.0.insert(k.into(), vec![v.into()]);
    }
    /// Go `Values.Add`.
    pub fn add(&mut self, k: impl Into<String>, v: impl Into<String>) {
        self.0.entry(k.into()).or_default().push(v.into());
    }
    /// Go `Values.Get`.
    pub fn get(&self, k: &str) -> &str {
        self.0.get(k).and_then(|v| v.first()).map(String::as_str).unwrap_or("")
    }
    /// Go `Values.Has`.
    pub fn has(&self, k: &str) -> bool {
        self.0.contains_key(k)
    }
    /// Go `Values.Del`.
    pub fn del(&mut self, k: &str) {
        self.0.remove(k);
    }
    /// Number of keys (Go `len(values)`).
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Reports whether there are no keys.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Iterates keys and their values in sorted key order.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<String>)> {
        self.0.iter()
    }
    /// Go `Values.Encode`.
    pub fn encode(&self) -> String {
        let mut out = String::new();
        for (k, vs) in &self.0 {
            let ke = query_escape(k);
            for v in vs {
                if !out.is_empty() {
                    out.push('&');
                }
                out.push_str(&ke);
                out.push('=');
                out.push_str(&query_escape(v));
            }
        }
        out
    }
    /// Go `url.ParseQuery`.
    pub fn parse(query: &str) -> Result<Self, String> {
        let mut out = Self::new();
        let mut first_err: Option<String> = None;
        for part in query.split('&') {
            if part.is_empty() {
                continue;
            }
            if part.contains(';') {
                first_err.get_or_insert_with(|| "invalid semicolon separator in query".into());
                continue;
            }
            let (k, v) = part.split_once('=').unwrap_or((part, ""));
            let k = match query_unescape(k) {
                Ok(k) => k,
                Err(e) => {
                    first_err.get_or_insert(e);
                    continue;
                }
            };
            let v = match query_unescape(v) {
                Ok(v) => v,
                Err(e) => {
                    first_err.get_or_insert(e);
                    continue;
                }
            };
            out.add(k, v);
        }
        match first_err {
            Some(e) => Err(e),
            None => Ok(out),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_match_go() {
        assert_eq!(path_escape("a b/c;d,e?f$&+:=@"), "a%20b%2Fc%3Bd%2Ce%3Ff$&+:=@");
        assert_eq!(query_escape("a b/c[]"), "a+b%2Fc%5B%5D");
        let mut v = Values::new();
        v.set("limit", "5");
        v.set("filter[app]", "1 2");
        assert_eq!(v.encode(), "filter%5Bapp%5D=1+2&limit=5");
    }
}
