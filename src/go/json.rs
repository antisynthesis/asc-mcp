//! Go `encoding/json` compatible decoding and encoding.
//!
//! The server was originally written in Go, and its wire behaviour (the
//! exact JSON bodies sent to App Store Connect, the `structuredContent`
//! returned to MCP clients, and the text of argument-decoding errors) is
//! defined by Go's `encoding/json` semantics. This module reproduces those
//! semantics on top of `serde_json::Value`:
//!
//! * Decoding walks object members in document order, matches keys exactly
//!   first and then ASCII case-insensitively, ignores unknown keys, treats
//!   `null` as a no-op for scalars and structs and as "reset to nil" for
//!   pointers, slices and maps, and keeps decoding after a type mismatch so
//!   that the *first* mismatch is reported, worded exactly as Go words it.
//! * Encoding emits struct fields in declaration order, honours
//!   `omitempty`, distinguishes nil from empty slices and maps, sorts map
//!   keys, and formats floats the way Go does.
//!
//! Types opt in through [`GoDecode`]/[`GoEncode`]; structs normally use the
//! [`go_struct!`](crate::go_struct) macro.

use std::collections::BTreeMap;
use std::fmt;
use std::ops::{Deref, DerefMut};

use serde_json::{Map, Number, Value};

use super::time::GoTime;

/// An error that aborts decoding immediately (Go returns errors from
/// `UnmarshalJSON` implementations without saving them).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fatal(pub String);

/// Result of a decode step.
pub type DResult = Result<(), Fatal>;

/// Decoding state: the error context Go attaches to type mismatches plus
/// the first saved mismatch.
#[derive(Debug, Default)]
pub struct Decoder {
    struct_name: &'static str,
    path: Vec<&'static str>,
    saved: Option<String>,
    type_override: Option<&'static str>,
}

impl Decoder {
    /// Creates a decoder with no context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a type mismatch (only the first one is kept, as in Go).
    pub fn type_error(&mut self, value_desc: &str, go_type: &str) {
        if self.saved.is_some() {
            return;
        }
        let go_type = self.type_override.unwrap_or(go_type);
        let msg = if self.path.is_empty() {
            format!("json: cannot unmarshal {value_desc} into Go value of type {go_type}")
        } else {
            format!(
                "json: cannot unmarshal {value_desc} into Go struct field {}.{} of type {go_type}",
                self.struct_name,
                self.path.join(".")
            )
        };
        self.saved = Some(msg);
    }

    /// Decodes a struct field: pushes the error context, decodes, pops.
    pub fn field<T: GoDecode + ?Sized>(
        &mut self,
        struct_short: &'static str,
        json: &'static str,
        type_override: Option<&'static str>,
        target: &mut T,
        v: &Value,
    ) -> DResult {
        let prev_struct = self.struct_name;
        let prev_override = self.type_override;
        self.struct_name = struct_short;
        self.path.push(json);
        self.type_override = type_override;
        let r = target.go_decode(v, self);
        self.type_override = prev_override;
        self.path.pop();
        self.struct_name = prev_struct;
        r
    }

    /// Pushes an embedded struct's Go field name onto the error path.
    #[doc(hidden)]
    pub fn push_path(&mut self, name: &'static str) {
        self.path.push(name);
    }

    /// Pops the most recent error path element.
    #[doc(hidden)]
    pub fn pop_path(&mut self) {
        self.path.pop();
    }

    /// Clears any per-field type override before descending into a
    /// composite value (overrides apply only to the field's own scalar).
    fn nested<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let prev = self.type_override.take();
        let r = f(self);
        self.type_override = prev;
        r
    }

    /// Returns the first saved error, if any.
    pub fn finish(self) -> Result<(), String> {
        match self.saved {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

/// The Go `UnmarshalTypeError.Value` description of a JSON value kind.
pub fn kind(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// Types that decode with Go `encoding/json` semantics.
pub trait GoDecode {
    /// The Go type name used in error messages (`reflect.Type.String()`).
    fn go_type() -> String
    where
        Self: Sized;

    /// Decodes `v` into `self`, merging as Go does.
    fn go_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult;
}

/// Types that encode with Go `encoding/json` semantics.
pub trait GoEncode {
    /// Encodes `self` as a JSON value.
    fn go_encode(&self) -> Value;

    /// Reports whether the value is "empty" for `omitempty`.
    fn go_is_empty(&self) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// Entry points

/// Go `json.Unmarshal(data, out)`.
pub fn unmarshal<T: GoDecode>(data: &[u8], out: &mut T) -> Result<(), String> {
    let v = parse(data)?;
    unmarshal_value(&v, out)
}

/// Go `json.Unmarshal` from an already parsed value.
pub fn unmarshal_value<T: GoDecode>(v: &Value, out: &mut T) -> Result<(), String> {
    let mut d = Decoder::new();
    out.go_decode(v, &mut d).map_err(|f| f.0)?;
    d.finish()
}

/// Go `json.Unmarshal(args, out)` where `args` is a `json.RawMessage`
/// that may be nil (absent).
pub fn unmarshal_args<T: GoDecode>(args: Option<&Value>, out: &mut T) -> Result<(), String> {
    match args {
        None => Err("unexpected end of JSON input".to_string()),
        Some(v) => unmarshal_value(v, out),
    }
}

/// Decodes a fresh `T` from `data`.
pub fn from_slice<T: GoDecode + Default>(data: &[u8]) -> Result<T, String> {
    let mut out = T::default();
    unmarshal(data, &mut out)?;
    Ok(out)
}

/// Go `json.Marshal(v)` as a value.
pub fn to_value<T: GoEncode + ?Sized>(v: &T) -> Value {
    v.go_encode()
}

/// Go `json.Marshal(v)` as bytes. Like Go, `<`, `>` and `&` are escaped.
pub fn marshal<T: GoEncode + ?Sized>(v: &T) -> Vec<u8> {
    to_go_json_bytes(&v.go_encode())
}

/// Serializes a value the way Go's encoder writes bytes (HTML-safe
/// escaping, no whitespace).
pub fn to_go_json_bytes(v: &Value) -> Vec<u8> {
    let s = serde_json::to_string(v).unwrap_or_else(|_| "null".into());
    html_escape(&s).into_bytes()
}

/// Escapes `<`, `>`, `&`, U+2028 and U+2029 like Go's encoder. Those
/// characters can only appear inside JSON strings, so a plain replacement
/// over serialized JSON is safe.
pub fn html_escape(s: &str) -> String {
    if !s.contains(['<', '>', '&', '\u{2028}', '\u{2029}']) {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            c => out.push(c),
        }
    }
    out
}

/// Parses bytes as JSON, producing Go-worded syntax errors.
pub fn parse(data: &[u8]) -> Result<Value, String> {
    if data.iter().all(|b| b.is_ascii_whitespace()) {
        return Err("unexpected end of JSON input".into());
    }
    serde_json::from_slice::<Value>(data).map_err(|e| go_syntax_error(data, &e))
}

/// Best-effort translation of a serde syntax error to Go's wording.
fn go_syntax_error(data: &[u8], e: &serde_json::Error) -> String {
    if e.is_eof() {
        return "unexpected end of JSON input".into();
    }
    // Locate the offending byte from serde's line/column.
    let (line, col) = (e.line(), e.column());
    let mut cur_line = 1;
    let mut offset = 0;
    for (i, b) in data.iter().enumerate() {
        if cur_line == line {
            offset = i + col.saturating_sub(1);
            break;
        }
        if *b == b'\n' {
            cur_line += 1;
        }
    }
    let c = data.get(offset).copied().unwrap_or(b' ');
    let first = data.iter().position(|b| !b.is_ascii_whitespace()).unwrap_or(0);
    let quoted = quote_char(c);
    if offset == first {
        format!("invalid character {quoted} looking for beginning of value")
    } else if e.to_string().contains("trailing characters") {
        format!("invalid character {quoted} after top-level value")
    } else {
        format!("invalid character {quoted} looking for beginning of value")
    }
}

fn quote_char(c: u8) -> String {
    match c {
        b'\'' => "'\\''".into(),
        b'"' => "'\"'".into(),
        c if c.is_ascii_graphic() || c == b' ' => format!("'{}'", c as char),
        c => format!("'\\x{c:02x}'"),
    }
}

// ---------------------------------------------------------------------------
// Scalars

impl GoDecode for String {
    fn go_type() -> String {
        "string".into()
    }
    fn go_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match v {
            Value::Null => {}
            Value::String(s) => *self = s.clone(),
            other => d.type_error(kind(other), "string"),
        }
        Ok(())
    }
}
impl GoEncode for String {
    fn go_encode(&self) -> Value {
        Value::String(self.clone())
    }
    fn go_is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl GoEncode for str {
    fn go_encode(&self) -> Value {
        Value::String(self.to_string())
    }
    fn go_is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl GoDecode for bool {
    fn go_type() -> String {
        "bool".into()
    }
    fn go_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match v {
            Value::Null => {}
            Value::Bool(b) => *self = *b,
            other => d.type_error(kind(other), "bool"),
        }
        Ok(())
    }
}
impl GoEncode for bool {
    fn go_encode(&self) -> Value {
        Value::Bool(*self)
    }
    fn go_is_empty(&self) -> bool {
        !*self
    }
}

/// Go `int` (64-bit). Use the `as "int64"` field flag for `int64`.
impl GoDecode for i64 {
    fn go_type() -> String {
        "int".into()
    }
    fn go_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match v {
            Value::Null => {}
            Value::Number(n) => {
                let lit = n.to_string();
                match lit.parse::<i64>() {
                    Ok(i) => *self = i,
                    Err(_) => d.type_error(&format!("number {lit}"), "int"),
                }
            }
            other => d.type_error(kind(other), "int"),
        }
        Ok(())
    }
}
impl GoEncode for i64 {
    fn go_encode(&self) -> Value {
        Value::Number(Number::from(*self))
    }
    fn go_is_empty(&self) -> bool {
        *self == 0
    }
}

impl GoDecode for f64 {
    fn go_type() -> String {
        "float64".into()
    }
    fn go_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match v {
            Value::Null => {}
            Value::Number(n) => {
                let lit = n.to_string();
                match lit.parse::<f64>() {
                    Ok(f) if f.is_finite() => *self = f,
                    _ => d.type_error(&format!("number {lit}"), "float64"),
                }
            }
            other => d.type_error(kind(other), "float64"),
        }
        Ok(())
    }
}
impl GoEncode for f64 {
    fn go_encode(&self) -> Value {
        go_float_number(*self)
    }
    fn go_is_empty(&self) -> bool {
        *self == 0.0
    }
}

/// Formats a float64 exactly as Go's JSON encoder does.
pub fn go_float_json(f: f64) -> String {
    if f == 0.0 {
        return if f.is_sign_negative() { "-0".into() } else { "0".into() };
    }
    let abs = f.abs();
    if (1e-6..1e21).contains(&abs) {
        // 'f' format with the shortest round-trip digits.
        let s = format!("{f}");
        return s;
    }
    // 'e' format, shortest digits, Go-style exponent: e-07 → e-7, e+21.
    let s = format!("{f:e}"); // like 1.5e-7 or 1e21
    let (mant, exp) = s.split_once('e').unwrap_or((&s, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let sign = if exp < 0 { '-' } else { '+' };
    format!("{mant}e{sign}{}", exp.abs())
}

/// A JSON number holding a Go-formatted float64.
pub fn go_float_number(f: f64) -> Value {
    serde_json::from_str::<Number>(&go_float_json(f))
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

// ---------------------------------------------------------------------------
// Pointers (Option)

impl<T: GoDecode + Default> GoDecode for Option<T> {
    fn go_type() -> String {
        T::go_type()
    }
    fn go_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        if v.is_null() {
            *self = None;
            return Ok(());
        }
        self.get_or_insert_with(T::default).go_decode(v, d)
    }
}
impl<T: GoEncode> GoEncode for Option<T> {
    fn go_encode(&self) -> Value {
        match self {
            None => Value::Null,
            Some(v) => v.go_encode(),
        }
    }
    fn go_is_empty(&self) -> bool {
        self.is_none()
    }
}

impl<T: GoDecode + Default> GoDecode for Box<T> {
    fn go_type() -> String {
        T::go_type()
    }
    fn go_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        (**self).go_decode(v, d)
    }
}
impl<T: GoEncode> GoEncode for Box<T> {
    fn go_encode(&self) -> Value {
        (**self).go_encode()
    }
    fn go_is_empty(&self) -> bool {
        (**self).go_is_empty()
    }
}

// ---------------------------------------------------------------------------
// Slices

/// A Go slice: a `Vec` that remembers whether it is nil.
///
/// Derefs to `Vec<T>`, so handler code can iterate and index it directly.
/// Without `omitempty`, a nil slice encodes as `null` and an empty one as
/// `[]`, exactly as in Go.
#[derive(Clone, PartialEq)]
pub struct GoSlice<T> {
    items: Vec<T>,
    nil: bool,
}

impl<T> Default for GoSlice<T> {
    fn default() -> Self {
        Self { items: Vec::new(), nil: true }
    }
}

impl<T: fmt::Debug> fmt::Debug for GoSlice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nil {
            f.write_str("nil")
        } else {
            self.items.fmt(f)
        }
    }
}

impl<T> GoSlice<T> {
    /// A nil slice.
    pub fn nil() -> Self {
        Self::default()
    }
    /// A non-nil slice holding `items` (may be empty).
    pub fn from_vec(items: Vec<T>) -> Self {
        Self { items, nil: false }
    }
    /// Reports whether the slice is nil.
    pub fn is_nil(&self) -> bool {
        self.nil
    }
    /// Go `append`: appending to a nil slice makes it non-nil.
    pub fn push(&mut self, v: T) {
        self.nil = false;
        self.items.push(v);
    }
    /// Consumes the slice, returning its items.
    pub fn into_vec(self) -> Vec<T> {
        self.items
    }
}

impl<T> From<Vec<T>> for GoSlice<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_vec(v)
    }
}

impl<T> FromIterator<T> for GoSlice<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().collect())
    }
}

impl<T> Deref for GoSlice<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.items
    }
}
impl<T> DerefMut for GoSlice<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        self.nil = false;
        &mut self.items
    }
}

impl<'a, T> IntoIterator for &'a GoSlice<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
impl<T> IntoIterator for GoSlice<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T: GoDecode + Default> GoDecode for GoSlice<T> {
    fn go_type() -> String {
        format!("[]{}", T::go_type())
    }
    fn go_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match v {
            Value::Null => *self = Self::nil(),
            Value::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                d.nested(|d| {
                    for item in items {
                        let mut t = T::default();
                        t.go_decode(item, d)?;
                        out.push(t);
                    }
                    Ok::<(), Fatal>(())
                })?;
                *self = Self::from_vec(out);
            }
            other => d.type_error(kind(other), &Self::go_type()),
        }
        Ok(())
    }
}
impl<T: GoEncode> GoEncode for GoSlice<T> {
    fn go_encode(&self) -> Value {
        if self.nil {
            Value::Null
        } else {
            Value::Array(self.items.iter().map(GoEncode::go_encode).collect())
        }
    }
    fn go_is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Go `[]byte`: base64 in JSON.
#[derive(Clone, Default, PartialEq, Debug)]
pub struct GoBytes(pub Option<Vec<u8>>);

impl GoDecode for GoBytes {
    fn go_type() -> String {
        "[]uint8".into()
    }
    fn go_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        use base64::Engine;
        match v {
            Value::Null => self.0 = None,
            Value::String(s) => match base64::engine::general_purpose::STANDARD.decode(s) {
                Ok(b) => self.0 = Some(b),
                Err(e) => {
                    let pos = match e {
                        base64::DecodeError::InvalidByte(p, _) => p,
                        base64::DecodeError::InvalidLength(p) => p,
                        base64::DecodeError::InvalidLastSymbol(p, _) => p,
                        base64::DecodeError::InvalidPadding => s.len(),
                    };
                    return Err(Fatal(format!("illegal base64 data at input byte {pos}")));
                }
            },
            other => d.type_error(kind(other), "[]uint8"),
        }
        Ok(())
    }
}
impl GoEncode for GoBytes {
    fn go_encode(&self) -> Value {
        use base64::Engine;
        match &self.0 {
            None => Value::Null,
            Some(b) => Value::String(base64::engine::general_purpose::STANDARD.encode(b)),
        }
    }
    fn go_is_empty(&self) -> bool {
        self.0.as_ref().is_none_or(|b| b.is_empty())
    }
}

// ---------------------------------------------------------------------------
// Maps

/// A Go `map[string]T`: sorted keys on encode, nil-aware.
#[derive(Clone, PartialEq)]
pub struct GoMap<T> {
    map: BTreeMap<String, T>,
    nil: bool,
}

impl<T> Default for GoMap<T> {
    fn default() -> Self {
        Self { map: BTreeMap::new(), nil: true }
    }
}

impl<T: fmt::Debug> fmt::Debug for GoMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nil {
            f.write_str("nil")
        } else {
            self.map.fmt(f)
        }
    }
}

impl<T> GoMap<T> {
    /// A nil map.
    pub fn nil() -> Self {
        Self::default()
    }
    /// A non-nil map with the given entries.
    pub fn from_map(map: BTreeMap<String, T>) -> Self {
        Self { map, nil: false }
    }
    /// Reports whether the map is nil.
    pub fn is_nil(&self) -> bool {
        self.nil
    }
    /// Go `m[k] = v` (the map must be non-nil in Go; here it becomes so).
    pub fn insert(&mut self, k: impl Into<String>, v: T) {
        self.nil = false;
        self.map.insert(k.into(), v);
    }
}

impl<T> FromIterator<(String, T)> for GoMap<T> {
    fn from_iter<I: IntoIterator<Item = (String, T)>>(iter: I) -> Self {
        Self::from_map(iter.into_iter().collect())
    }
}

impl<T> Deref for GoMap<T> {
    type Target = BTreeMap<String, T>;
    fn deref(&self) -> &BTreeMap<String, T> {
        &self.map
    }
}
impl<T> DerefMut for GoMap<T> {
    fn deref_mut(&mut self) -> &mut BTreeMap<String, T> {
        self.nil = false;
        &mut self.map
    }
}

impl<T: GoDecode + Default> GoDecode for GoMap<T> {
    fn go_type() -> String {
        format!("map[string]{}", T::go_type())
    }
    fn go_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match v {
            Value::Null => *self = Self::nil(),
            Value::Object(m) => {
                self.nil = false;
                d.nested(|d| {
                    for (k, item) in m {
                        let mut t = T::default();
                        t.go_decode(item, d)?;
                        self.map.insert(k.clone(), t);
                    }
                    Ok::<(), Fatal>(())
                })?;
            }
            other => d.type_error(kind(other), &Self::go_type()),
        }
        Ok(())
    }
}
impl<T: GoEncode> GoEncode for GoMap<T> {
    fn go_encode(&self) -> Value {
        if self.nil {
            return Value::Null;
        }
        let mut m = Map::new();
        for (k, v) in &self.map {
            m.insert(k.clone(), v.go_encode());
        }
        Value::Object(m)
    }
    fn go_is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

// ---------------------------------------------------------------------------
// interface{} and json.RawMessage

/// Go `any` / `interface{}`: decoding converts numbers to float64, and
/// encoding sorts object keys (they became `map[string]any`).
impl GoDecode for Value {
    fn go_type() -> String {
        "interface {}".into()
    }
    fn go_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match normalize_any(v) {
            Ok(n) => *self = n,
            Err(lit) => d.type_error(&format!("number {lit}"), "float64"),
        }
        Ok(())
    }
}
impl GoEncode for Value {
    fn go_encode(&self) -> Value {
        sort_keys(self)
    }
    fn go_is_empty(&self) -> bool {
        self.is_null()
    }
}

/// Converts a JSON value to what Go's `any` would hold after decoding:
/// every number becomes a float64.
pub fn normalize_any(v: &Value) -> Result<Value, String> {
    Ok(match v {
        Value::Number(n) => {
            let lit = n.to_string();
            match lit.parse::<f64>() {
                Ok(f) if f.is_finite() => go_float_number(f),
                _ => return Err(lit),
            }
        }
        Value::Array(a) => Value::Array(a.iter().map(normalize_any).collect::<Result<_, _>>()?),
        Value::Object(m) => {
            let mut out = Map::new();
            for (k, v) in m {
                out.insert(k.clone(), normalize_any(v)?);
            }
            Value::Object(out)
        }
        other => other.clone(),
    })
}

/// Recursively sorts object keys (Go encodes maps with sorted keys).
pub fn sort_keys(v: &Value) -> Value {
    match v {
        Value::Array(a) => Value::Array(a.iter().map(sort_keys).collect()),
        Value::Object(m) => {
            let mut keys: Vec<&String> = m.keys().collect();
            keys.sort();
            let mut out = Map::new();
            for k in keys {
                out.insert(k.clone(), sort_keys(&m[k]));
            }
            Value::Object(out)
        }
        other => other.clone(),
    }
}

/// Go `json.RawMessage`: kept verbatim (nil when absent).
#[derive(Clone, Default, PartialEq, Debug)]
pub struct RawMessage(pub Option<Value>);

impl GoDecode for RawMessage {
    fn go_type() -> String {
        "json.RawMessage".into()
    }
    fn go_decode(&mut self, v: &Value, _d: &mut Decoder) -> DResult {
        self.0 = Some(v.clone());
        Ok(())
    }
}
impl GoEncode for RawMessage {
    fn go_encode(&self) -> Value {
        self.0.clone().unwrap_or(Value::Null)
    }
    fn go_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

// ---------------------------------------------------------------------------
// time.Time

impl GoDecode for GoTime {
    fn go_type() -> String {
        "time.Time".into()
    }
    fn go_decode(&mut self, v: &Value, _d: &mut Decoder) -> DResult {
        match v {
            Value::Null => Ok(()),
            Value::String(s) => {
                *self = GoTime::parse_rfc3339(s).map_err(Fatal)?;
                Ok(())
            }
            _ => Err(Fatal("Time.UnmarshalJSON: input is not a JSON string".into())),
        }
    }
}
impl GoEncode for GoTime {
    fn go_encode(&self) -> Value {
        Value::String(self.format_rfc3339_nano())
    }
}

// ---------------------------------------------------------------------------
// Structs

/// Matches an object key against a field's JSON name the way Go does:
/// exact first, then case-insensitively.
#[doc(hidden)]
pub fn key_matches(key: &str, json: &str, exact_pass: bool) -> bool {
    if exact_pass {
        key == json
    } else {
        key.eq_ignore_ascii_case(json)
    }
}

/// Struct-level decoding hooks used by [`go_struct!`](crate::go_struct),
/// including promotion of embedded (`flatten`) struct fields.
pub trait GoStruct {
    /// Tries to decode object member `k` into a field. `short` is the
    /// struct name Go reports in errors (the outermost struct for
    /// promoted fields). Returns whether a field matched.
    fn go_decode_member(
        &mut self,
        short: &'static str,
        k: &str,
        v: &Value,
        d: &mut Decoder,
        exact: bool,
    ) -> Result<bool, Fatal>;

    /// Writes this struct's fields into `m` in declaration order.
    fn go_encode_into(&self, m: &mut Map<String, Value>);
}

/// Defines a struct with Go `encoding/json` semantics.
///
/// ```ignore
/// go_struct! {
///     /// Doc comment.
///     pub struct AppAttributes ("api.AppAttributes", "AppAttributes") {
///         pub name: String => "name", omitempty;
///         pub file_size: i64 => "fileSize" as "int64";
///         pub created: Option<GoTime> => "createdDate", omitempty;
///         pub base: OtherStruct => "", flatten;   // Go embedded field
///     }
/// }
/// ```
///
/// The first string is Go's `reflect` type name (used when a non-object
/// is decoded into the struct at top level); the second is the short
/// struct name Go puts in field error messages (`""` for anonymous
/// structs). `as "int64"` overrides the Go type name reported for a
/// scalar field. `flatten` marks a Go embedded struct whose fields are
/// promoted into this struct's JSON object.
#[macro_export]
macro_rules! go_struct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident ($gotype:expr, $short:expr) {
            $(
                $(#[$fmeta:meta])*
                $fvis:vis $field:ident : $fty:ty => $json:literal $(as $ovr:literal)? $(, $flag:ident)* ;
            )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Default)]
        $vis struct $name {
            $( $(#[$fmeta])* $fvis $field : $fty, )*
        }

        impl $crate::go::json::GoStruct for $name {
            #[allow(unused_variables)]
            fn go_decode_member(
                &mut self,
                short: &'static str,
                k: &str,
                v: &::serde_json::Value,
                d: &mut $crate::go::json::Decoder,
                exact: bool,
            ) -> Result<bool, $crate::go::json::Fatal> {
                $(
                    $crate::go_struct!(@decode self, short, k, v, d, exact, $field, $json, [$($ovr)?] $(, $flag)*);
                )*
                Ok(false)
            }

            #[allow(unused_variables)]
            fn go_encode_into(&self, m: &mut ::serde_json::Map<String, ::serde_json::Value>) {
                $(
                    $crate::go_struct!(@encode m, self, $field, $json $(, $flag)*);
                )*
            }
        }

        impl $crate::go::json::GoDecode for $name {
            fn go_type() -> String { ($gotype).to_string() }
            fn go_decode(
                &mut self,
                v: &::serde_json::Value,
                d: &mut $crate::go::json::Decoder,
            ) -> $crate::go::json::DResult {
                match v {
                    ::serde_json::Value::Null => Ok(()),
                    ::serde_json::Value::Object(m) => {
                        for (k, val) in m {
                            for exact in [true, false] {
                                if $crate::go::json::GoStruct::go_decode_member(self, $short, k, val, d, exact)? {
                                    break;
                                }
                            }
                        }
                        Ok(())
                    }
                    other => {
                        d.type_error($crate::go::json::kind(other), &<Self as $crate::go::json::GoDecode>::go_type());
                        Ok(())
                    }
                }
            }
        }

        impl $crate::go::json::GoEncode for $name {
            fn go_encode(&self) -> ::serde_json::Value {
                let mut m = ::serde_json::Map::new();
                $crate::go::json::GoStruct::go_encode_into(self, &mut m);
                ::serde_json::Value::Object(m)
            }
        }
    };

    (@decode $s:ident, $short:ident, $k:ident, $v:ident, $d:ident, $exact:ident, $field:ident, $json:literal, [$($ovr:literal)?] , flatten) => {
        $d.push_path($json);
        let matched = $crate::go::json::GoStruct::go_decode_member(&mut $s.$field, $short, $k, $v, $d, $exact);
        $d.pop_path();
        if matched? {
            return Ok(true);
        }
    };
    (@decode $s:ident, $short:ident, $k:ident, $v:ident, $d:ident, $exact:ident, $field:ident, $json:literal, [$($ovr:literal)?] $(, $flag:ident)*) => {
        if $crate::go::json::key_matches($k, $json, $exact) {
            let ovr: Option<&'static str> = None $(.or(Some($ovr)))?;
            $d.field($short, $json, ovr, &mut $s.$field, $v)?;
            return Ok(true);
        }
    };

    (@encode $m:ident, $s:ident, $field:ident, $json:literal, flatten) => {
        $crate::go::json::GoStruct::go_encode_into(&$s.$field, $m);
    };
    (@encode $m:ident, $s:ident, $field:ident, $json:literal) => {
        $m.insert($json.to_string(), $crate::go::json::GoEncode::go_encode(&$s.$field));
    };
    (@encode $m:ident, $s:ident, $field:ident, $json:literal, omitempty) => {
        if !$crate::go::json::GoEncode::go_is_empty(&$s.$field) {
            $m.insert($json.to_string(), $crate::go::json::GoEncode::go_encode(&$s.$field));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::go::time::GoTime;

    go_struct! {
        struct Inner ("main.Inner", "Inner") {
            n: i64 => "n";
        }
    }
    go_struct! {
        struct Named ("main.Named", "Named") {
            s: String => "s";
            i: i64 => "i";
            i64v: i64 => "i64" as "int64";
            f: f64 => "f";
            b: bool => "b";
            p: Option<i64> => "p";
            l: GoSlice<String> => "l";
            m: GoMap<GoSlice<String>> => "m";
            inner: Inner => "in";
            ins: GoSlice<Inner> => "ins";
            t: Option<GoTime> => "t";
            tt: GoTime => "tt";
            a: Value => "a";
            bs: GoBytes => "bs";
        }
    }

    fn err(js: &str) -> String {
        let mut v = Named::default();
        match unmarshal(js.as_bytes(), &mut v) {
            Ok(()) => "<nil>".into(),
            Err(e) => e,
        }
    }

    #[test]
    fn go_error_messages() {
        let cases = [
            (r#"{"s":1}"#, "json: cannot unmarshal number into Go struct field Named.s of type string"),
            (r#"{"i":"x"}"#, "json: cannot unmarshal string into Go struct field Named.i of type int"),
            (r#"{"i":1.5}"#, "json: cannot unmarshal number 1.5 into Go struct field Named.i of type int"),
            (r#"{"i":5.0}"#, "json: cannot unmarshal number 5.0 into Go struct field Named.i of type int"),
            // serde_json normalises exponent literals, so Go's "number 1e30" reads "number 1e+30" here.
            (r#"{"i":1e30}"#, "json: cannot unmarshal number 1e+30 into Go struct field Named.i of type int"),
            (r#"{"i64":"x"}"#, "json: cannot unmarshal string into Go struct field Named.i64 of type int64"),
            (r#"{"f":"x"}"#, "json: cannot unmarshal string into Go struct field Named.f of type float64"),
            (r#"{"b":1}"#, "json: cannot unmarshal number into Go struct field Named.b of type bool"),
            (r#"{"p":"x"}"#, "json: cannot unmarshal string into Go struct field Named.p of type int"),
            (r#"{"l":"x"}"#, "json: cannot unmarshal string into Go struct field Named.l of type []string"),
            (r#"{"l":[1]}"#, "json: cannot unmarshal number into Go struct field Named.l of type string"),
            (r#"{"m":{"k":"v"}}"#, "json: cannot unmarshal string into Go struct field Named.m of type []string"),
            (r#"{"m":{"k":[1]}}"#, "json: cannot unmarshal number into Go struct field Named.m of type string"),
            (r#"{"m":[]}"#, "json: cannot unmarshal array into Go struct field Named.m of type map[string][]string"),
            (r#"{"in":{"n":"x"}}"#, "json: cannot unmarshal string into Go struct field Inner.in.n of type int"),
            (r#"{"in":5}"#, "json: cannot unmarshal number into Go struct field Named.in of type main.Inner"),
            (r#"{"ins":[{"n":"x"}]}"#, "json: cannot unmarshal string into Go struct field Inner.ins.n of type int"),
            (r#"{"t":"bad"}"#, r#"parsing time "bad" as "2006-01-02T15:04:05Z07:00": cannot parse "bad" as "2006""#),
            (r#"{"t":5}"#, "Time.UnmarshalJSON: input is not a JSON string"),
            (r#"{"tt":"2024-01-01"}"#, r#"parsing time "2024-01-01" as "2006-01-02T15:04:05Z07:00": cannot parse "" as "T""#),
            (r#"{"t":"2024-01-01T10:00:00+0000"}"#, r#"parsing time "2024-01-01T10:00:00+0000" as "2006-01-02T15:04:05Z07:00": cannot parse "+0000" as "Z07:00""#),
            (r#"{"bs":"!!"}"#, "illegal base64 data at input byte 0"),
            (r#"{"bs":5}"#, "json: cannot unmarshal number into Go struct field Named.bs of type []uint8"),
            ("[1]", "json: cannot unmarshal array into Go value of type main.Named"),
            (r#""x""#, "json: cannot unmarshal string into Go value of type main.Named"),
            ("", "unexpected end of JSON input"),
            (r#"{"i":"x","s":2}"#, "json: cannot unmarshal string into Go struct field Named.i of type int"),
            (r#"{"S":"case"}"#, "<nil>"),
        ];
        for (js, want) in cases {
            assert_eq!(err(js), want, "input {js}");
        }
    }

    #[test]
    fn embedded_fields_are_promoted() {
        go_struct! {
            struct Base ("tools.Base", "Base") {
                limit: i64 => "limit";
                cursor: String => "cursor";
            }
        }
        go_struct! {
            struct P ("struct { tools.Base; AppID string \"json:\\\"app_id\\\"\" }", "") {
                base: Base => "Base", flatten;
                app_id: String => "app_id";
            }
        }
        let mut p = P::default();
        unmarshal(br#"{"LIMIT":3,"app_id":"a","cursor":"c"}"#, &mut p).unwrap();
        assert_eq!((p.base.limit, p.base.cursor.as_str(), p.app_id.as_str()), (3, "c", "a"));
        let mut p = P::default();
        assert_eq!(
            unmarshal(br#"{"limit":"x"}"#, &mut p).unwrap_err(),
            "json: cannot unmarshal string into Go struct field .Base.limit of type int"
        );
        assert_eq!(String::from_utf8(marshal(&p)).unwrap(), r#"{"limit":0,"cursor":"","app_id":""}"#);
    }

    #[test]
    fn go_float_formatting() {
        let cases = [
            (0.0, "0"), (1.0, "1"), (1.5, "1.5"), (100.0, "100"), (1234567.0, "1234567"),
            (123456.7, "123456.7"), (1e21, "1e+21"), (1e20, "100000000000000000000"),
            (0.0001, "0.0001"), (0.00001, "0.00001"), (-2.5e-8, "-2.5e-8"), (100000000.0, "100000000"),
        ];
        for (f, want) in cases {
            assert_eq!(go_float_json(f), want);
        }
    }

    #[test]
    fn encode_semantics() {
        go_struct! {
            struct E ("main.E", "E") {
                z: GoTime => "z", omitempty;
                s: GoSlice<String> => "s";
                s2: GoSlice<String> => "s2", omitempty;
                m: GoMap<i64> => "m";
                a: Value => "a";
                p: Option<bool> => "p", omitempty;
                h: String => "h";
            }
        }
        let e = E { h: "<a&b>".into(), ..Default::default() };
        assert_eq!(
            String::from_utf8(marshal(&e)).unwrap(),
            r#"{"z":"0001-01-01T00:00:00Z","s":null,"m":null,"a":null,"h":"\u003ca\u0026b\u003e"}"#
        );
        let mut any = Value::Null;
        unmarshal(br#"{"b":1,"a":[1.50,12345678901234567890,"x",null,true],"c":{"z":1,"y":2}}"#, &mut any).unwrap();
        assert_eq!(
            String::from_utf8(marshal(&any)).unwrap(),
            r#"{"a":[1.5,12345678901234567000,"x",null,true],"b":1,"c":{"y":2,"z":1}}"#
        );
    }
}
