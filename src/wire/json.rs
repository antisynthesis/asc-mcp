//! JSON decoding and encoding with the server's wire-format rules.
//!
//! The server's wire behaviour (the exact JSON bodies sent to App Store
//! Connect, the `structuredContent` returned to MCP clients, and the text of
//! argument-decoding errors) is a stable format that clients depend on. This
//! module implements those rules on top of `serde_json::Value`:
//!
//! * Decoding walks object members in document order, matches keys exactly
//!   first and then ASCII case-insensitively, ignores unknown keys, treats
//!   `null` as a no-op for scalars and structs and as "reset to nil" for
//!   pointers, slices and maps, and keeps decoding after a type mismatch so
//!   that the *first* mismatch is reported, using the established error
//!   wording (`json: cannot unmarshal ... into Go struct field ...`).
//! * Encoding emits struct fields in declaration order, honours
//!   `omitempty`, distinguishes nil from empty slices and maps, sorts map
//!   keys, escapes `<`, `>` and `&` (HTML-safe), and formats floats with
//!   the shortest round-trip digits.
//!
//! Types opt in through [`JsonDecode`]/[`JsonEncode`]; structs normally use the
//! [`json_struct!`](crate::json_struct) macro.

use std::collections::BTreeMap;
use std::fmt;
use std::ops::{Deref, DerefMut};

use serde_json::{Map, Number, Value};

use super::time::Timestamp;

/// An error that aborts decoding immediately, unlike a type mismatch, which
/// is saved and decoding continues (used for invalid base64 data and
/// malformed timestamps).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fatal(pub String);

/// Result of a decode step.
pub type DResult = Result<(), Fatal>;

/// Decoding state: the struct/field context attached to type-mismatch
/// errors plus the first saved mismatch.
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

    /// Records a type mismatch (only the first one is kept).
    pub fn type_error(&mut self, value_desc: &str, type_name: &str) {
        if self.saved.is_some() {
            return;
        }
        let type_name = self.type_override.unwrap_or(type_name);
        let msg = if self.path.is_empty() {
            format!("json: cannot unmarshal {value_desc} into Go value of type {type_name}")
        } else {
            format!(
                "json: cannot unmarshal {value_desc} into Go struct field {}.{} of type {type_name}",
                self.struct_name,
                self.path.join(".")
            )
        };
        self.saved = Some(msg);
    }

    /// Decodes a struct field: pushes the error context, decodes, pops.
    pub fn field<T: JsonDecode + ?Sized>(
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
        let r = target.json_decode(v, self);
        self.type_override = prev_override;
        self.path.pop();
        self.struct_name = prev_struct;
        r
    }

    /// Pushes an embedded struct's field name onto the error path.
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

/// The description of a JSON value kind used in type-mismatch errors.
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

/// Types that decode with the wire-format rules.
pub trait JsonDecode {
    /// The type name used in error messages (e.g. `int`, `[]string`).
    fn type_name() -> String
    where
        Self: Sized;

    /// Decodes `v` into `self`, merging into existing contents (absent
    /// members leave fields untouched).
    fn json_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult;
}

/// Types that encode with the wire-format rules.
pub trait JsonEncode {
    /// Encodes `self` as a JSON value.
    fn json_encode(&self) -> Value;

    /// Reports whether the value is "empty" for `omitempty`.
    fn is_omittable(&self) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// Entry points

/// Parses `data` and decodes it into `out`.
pub fn unmarshal<T: JsonDecode>(data: &[u8], out: &mut T) -> Result<(), String> {
    let v = parse(data)?;
    unmarshal_value(&v, out)
}

/// Decodes an already parsed value into `out`.
pub fn unmarshal_value<T: JsonDecode>(v: &Value, out: &mut T) -> Result<(), String> {
    let mut d = Decoder::new();
    out.json_decode(v, &mut d).map_err(|f| f.0)?;
    d.finish()
}

/// Decodes tool arguments into `out`; absent arguments are reported as
/// `unexpected end of JSON input`.
pub fn unmarshal_args<T: JsonDecode>(args: Option<&Value>, out: &mut T) -> Result<(), String> {
    match args {
        None => Err("unexpected end of JSON input".to_string()),
        Some(v) => unmarshal_value(v, out),
    }
}

/// Decodes a fresh `T` from `data`.
pub fn from_slice<T: JsonDecode + Default>(data: &[u8]) -> Result<T, String> {
    let mut out = T::default();
    unmarshal(data, &mut out)?;
    Ok(out)
}

/// Encodes `v` as a JSON value.
pub fn to_value<T: JsonEncode + ?Sized>(v: &T) -> Value {
    v.json_encode()
}

/// Encodes `v` as compact bytes with `<`, `>` and `&` escaped.
pub fn marshal<T: JsonEncode + ?Sized>(v: &T) -> Vec<u8> {
    to_json_bytes(&v.json_encode())
}

/// Serializes a value to wire bytes (HTML-safe escaping, no whitespace).
pub fn to_json_bytes(v: &Value) -> Vec<u8> {
    let s = serde_json::to_string(v).unwrap_or_else(|_| "null".into());
    html_escape(&s).into_bytes()
}

/// Escapes `<`, `>`, `&`, U+2028 and U+2029 as `\uXXXX`. Those
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

/// Parses bytes as JSON, producing syntax errors in the established
/// wording (`invalid character ...`, `unexpected end of JSON input`).
pub fn parse(data: &[u8]) -> Result<Value, String> {
    if data.iter().all(|b| b.is_ascii_whitespace()) {
        return Err("unexpected end of JSON input".into());
    }
    serde_json::from_slice::<Value>(data).map_err(|e| syntax_error(data, &e))
}

/// Translates a syntax error into the established wording by re-scanning
/// the input with [`scan`].
fn syntax_error(data: &[u8], e: &serde_json::Error) -> String {
    match scan(data) {
        Err(msg) => msg,
        // The scanner accepted what serde rejected (e.g. invalid UTF-8,
        // which the scanner tolerates); fall back to serde's description.
        Ok(()) => e.to_string(),
    }
}

fn quote_char(c: u8) -> String {
    match c {
        b'\'' => "'\\''".into(),
        b'"' => "'\"'".into(),
        c => {
            let q = crate::wire::fmt::quote(&char::from(c).to_string());
            format!("'{}'", &q[1..q.len() - 1])
        }
    }
}

/// Validates JSON byte by byte, returning the established error text for
/// the first offending character.
pub fn scan(data: &[u8]) -> Result<(), String> {
    struct S<'a> {
        b: &'a [u8],
        i: usize,
        eof_space: bool,
    }
    const EOF: &str = "unexpected end of JSON input";
    fn bad(c: u8, ctx: &str) -> String {
        format!("invalid character {} {ctx}", quote_char(c))
    }
    impl S<'_> {
        fn ws(&mut self) {
            while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\n' | b'\r') {
                self.i += 1;
            }
        }
        /// Reads a byte. At end of input the scanner feeds one virtual
        /// space (so `nul` fails "in literal null"); a second read is EOF.
        fn next(&mut self) -> Result<u8, String> {
            match self.b.get(self.i) {
                Some(&c) => {
                    self.i += 1;
                    Ok(c)
                }
                None if !self.eof_space => {
                    self.eof_space = true;
                    Ok(b' ')
                }
                None => Err(EOF.to_string()),
            }
        }
        /// Skips whitespace, then reads a byte (EOF if none remains).
        fn next_ns(&mut self) -> Result<u8, String> {
            self.ws();
            if self.i >= self.b.len() {
                return Err(EOF.to_string());
            }
            self.next()
        }
        fn peek(&self) -> Option<u8> {
            self.b.get(self.i).copied()
        }
        fn literal(&mut self, rest: &[u8], name: &str) -> Result<(), String> {
            for &want in rest {
                let c = self.next()?;
                if c != want {
                    return Err(bad(
                        c,
                        &format!("in literal {name} (expecting {})", quote_char(want)),
                    ));
                }
            }
            Ok(())
        }
        fn digits(&mut self) {
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.i += 1;
            }
        }
        fn number(&mut self, first: u8) -> Result<(), String> {
            let mut c = first;
            if c == b'-' {
                c = self.next()?;
                if !c.is_ascii_digit() {
                    return Err(bad(c, "in numeric literal"));
                }
            }
            if c != b'0' {
                self.digits();
            }
            if self.peek() == Some(b'.') {
                self.i += 1;
                let c = self.next()?;
                if !c.is_ascii_digit() {
                    return Err(bad(c, "after decimal point in numeric literal"));
                }
                self.digits();
            }
            if matches!(self.peek(), Some(b'e' | b'E')) {
                self.i += 1;
                let mut c = self.next()?;
                if c == b'+' || c == b'-' {
                    c = self.next()?;
                }
                if !c.is_ascii_digit() {
                    return Err(bad(c, "in exponent of numeric literal"));
                }
                self.digits();
            }
            Ok(())
        }
        fn string(&mut self) -> Result<(), String> {
            loop {
                let c = self.next()?;
                match c {
                    b'"' => return Ok(()),
                    b'\\' => {
                        let e = self.next()?;
                        match e {
                            b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => {}
                            b'u' => {
                                for _ in 0..4 {
                                    let h = self.next()?;
                                    if !h.is_ascii_hexdigit() {
                                        return Err(bad(h, "in \\u hexadecimal character escape"));
                                    }
                                }
                            }
                            other => return Err(bad(other, "in string escape code")),
                        }
                    }
                    c if c < 0x20 => return Err(bad(c, "in string literal")),
                    _ => {}
                }
            }
        }
        fn value(&mut self) -> Result<(), String> {
            let c = self.next_ns()?;
            match c {
                b'{' => {
                    self.ws();
                    if self.peek() == Some(b'}') {
                        self.i += 1;
                        return Ok(());
                    }
                    loop {
                        let c = self.next_ns()?;
                        if c != b'"' {
                            return Err(bad(c, "looking for beginning of object key string"));
                        }
                        self.string()?;
                        let c = self.next_ns()?;
                        if c != b':' {
                            return Err(bad(c, "after object key"));
                        }
                        self.value()?;
                        match self.next_ns()? {
                            b',' => continue,
                            b'}' => return Ok(()),
                            c => return Err(bad(c, "after object key:value pair")),
                        }
                    }
                }
                b'[' => {
                    self.ws();
                    if self.peek() == Some(b']') {
                        self.i += 1;
                        return Ok(());
                    }
                    loop {
                        self.value()?;
                        match self.next_ns()? {
                            b',' => continue,
                            b']' => return Ok(()),
                            c => return Err(bad(c, "after array element")),
                        }
                    }
                }
                b'"' => self.string(),
                b'-' | b'0'..=b'9' => self.number(c),
                b't' => self.literal(b"rue", "true"),
                b'f' => self.literal(b"alse", "false"),
                b'n' => self.literal(b"ull", "null"),
                c => Err(bad(c, "looking for beginning of value")),
            }
        }
    }
    let mut s = S {
        b: data,
        i: 0,
        eof_space: false,
    };
    s.value()?;
    s.ws();
    if let Some(c) = s.peek() {
        return Err(bad(c, "after top-level value"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Scalars

impl JsonDecode for String {
    fn type_name() -> String {
        "string".into()
    }
    fn json_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match v {
            Value::Null => {}
            Value::String(s) => *self = s.clone(),
            other => d.type_error(kind(other), "string"),
        }
        Ok(())
    }
}
impl JsonEncode for String {
    fn json_encode(&self) -> Value {
        Value::String(self.clone())
    }
    fn is_omittable(&self) -> bool {
        self.is_empty()
    }
}
impl JsonEncode for str {
    fn json_encode(&self) -> Value {
        Value::String(self.to_string())
    }
    fn is_omittable(&self) -> bool {
        self.is_empty()
    }
}

impl JsonDecode for bool {
    fn type_name() -> String {
        "bool".into()
    }
    fn json_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match v {
            Value::Null => {}
            Value::Bool(b) => *self = *b,
            other => d.type_error(kind(other), "bool"),
        }
        Ok(())
    }
}
impl JsonEncode for bool {
    fn json_encode(&self) -> Value {
        Value::Bool(*self)
    }
    fn is_omittable(&self) -> bool {
        !*self
    }
}

/// A 64-bit integer, reported as `int` in errors. Use the `as "int64"`
/// field flag to report `int64` instead.
impl JsonDecode for i64 {
    fn type_name() -> String {
        "int".into()
    }
    fn json_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
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
impl JsonEncode for i64 {
    fn json_encode(&self) -> Value {
        Value::Number(Number::from(*self))
    }
    fn is_omittable(&self) -> bool {
        *self == 0
    }
}

impl JsonDecode for f64 {
    fn type_name() -> String {
        "float64".into()
    }
    fn json_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
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
impl JsonEncode for f64 {
    fn json_encode(&self) -> Value {
        float_number(*self)
    }
    fn is_omittable(&self) -> bool {
        *self == 0.0
    }
}

/// Formats a float64 for the wire: shortest round-trip digits, plain
/// notation for magnitudes in `[1e-6, 1e21)`, exponent notation otherwise.
pub fn float_json(f: f64) -> String {
    if f == 0.0 {
        return if f.is_sign_negative() {
            "-0".into()
        } else {
            "0".into()
        };
    }
    let abs = f.abs();
    if (1e-6..1e21).contains(&abs) {
        // 'f' format with the shortest round-trip digits.
        let s = format!("{f}");
        return s;
    }
    // 'e' format, shortest digits, signed exponent without padding: e-7, e+21.
    let s = format!("{f:e}"); // like 1.5e-7 or 1e21
    let (mant, exp) = s.split_once('e').unwrap_or((&s, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let sign = if exp < 0 { '-' } else { '+' };
    format!("{mant}e{sign}{}", exp.abs())
}

/// A JSON number holding a float64 formatted by [`float_json`].
pub fn float_number(f: f64) -> Value {
    serde_json::from_str::<Number>(&float_json(f))
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

// ---------------------------------------------------------------------------
// Pointers (Option)

impl<T: JsonDecode + Default> JsonDecode for Option<T> {
    fn type_name() -> String {
        T::type_name()
    }
    fn json_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        if v.is_null() {
            *self = None;
            return Ok(());
        }
        self.get_or_insert_with(T::default).json_decode(v, d)
    }
}
impl<T: JsonEncode> JsonEncode for Option<T> {
    fn json_encode(&self) -> Value {
        match self {
            None => Value::Null,
            Some(v) => v.json_encode(),
        }
    }
    fn is_omittable(&self) -> bool {
        self.is_none()
    }
}

impl<T: JsonDecode + Default> JsonDecode for Box<T> {
    fn type_name() -> String {
        T::type_name()
    }
    fn json_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        (**self).json_decode(v, d)
    }
}
impl<T: JsonEncode> JsonEncode for Box<T> {
    fn json_encode(&self) -> Value {
        (**self).json_encode()
    }
    fn is_omittable(&self) -> bool {
        (**self).is_omittable()
    }
}

// ---------------------------------------------------------------------------
// Slices

/// A slice: a `Vec` that remembers whether it is nil.
///
/// Derefs to `Vec<T>`, so handler code can iterate and index it directly.
/// Without `omitempty`, a nil slice encodes as `null` and an empty one as
/// `[]`.
#[derive(Clone, PartialEq)]
pub struct JsonSlice<T> {
    items: Vec<T>,
    nil: bool,
}

impl<T> Default for JsonSlice<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            nil: true,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for JsonSlice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nil {
            f.write_str("nil")
        } else {
            self.items.fmt(f)
        }
    }
}

impl<T> JsonSlice<T> {
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
    /// Appends an item; appending to a nil slice makes it non-nil.
    pub fn push(&mut self, v: T) {
        self.nil = false;
        self.items.push(v);
    }
    /// Consumes the slice, returning its items.
    pub fn into_vec(self) -> Vec<T> {
        self.items
    }
}

impl<T> From<Vec<T>> for JsonSlice<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_vec(v)
    }
}

impl<T> FromIterator<T> for JsonSlice<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().collect())
    }
}

impl<T> Deref for JsonSlice<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.items
    }
}
impl<T> DerefMut for JsonSlice<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        self.nil = false;
        &mut self.items
    }
}

impl<'a, T> IntoIterator for &'a JsonSlice<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
impl<T> IntoIterator for JsonSlice<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T: JsonDecode + Default> JsonDecode for JsonSlice<T> {
    fn type_name() -> String {
        format!("[]{}", T::type_name())
    }
    fn json_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match v {
            Value::Null => *self = Self::nil(),
            Value::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                d.nested(|d| {
                    for item in items {
                        let mut t = T::default();
                        t.json_decode(item, d)?;
                        out.push(t);
                    }
                    Ok::<(), Fatal>(())
                })?;
                *self = Self::from_vec(out);
            }
            other => d.type_error(kind(other), &Self::type_name()),
        }
        Ok(())
    }
}
impl<T: JsonEncode> JsonEncode for JsonSlice<T> {
    fn json_encode(&self) -> Value {
        if self.nil {
            Value::Null
        } else {
            Value::Array(self.items.iter().map(JsonEncode::json_encode).collect())
        }
    }
    fn is_omittable(&self) -> bool {
        self.items.is_empty()
    }
}

/// Raw bytes, carried as standard base64 in JSON (nil when absent).
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Base64Bytes(pub Option<Vec<u8>>);

impl JsonDecode for Base64Bytes {
    fn type_name() -> String {
        "[]uint8".into()
    }
    fn json_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
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
impl JsonEncode for Base64Bytes {
    fn json_encode(&self) -> Value {
        use base64::Engine;
        match &self.0 {
            None => Value::Null,
            Some(b) => Value::String(base64::engine::general_purpose::STANDARD.encode(b)),
        }
    }
    fn is_omittable(&self) -> bool {
        self.0.as_ref().is_none_or(|b| b.is_empty())
    }
}

// ---------------------------------------------------------------------------
// Maps

/// A string-keyed map: sorted keys on encode, nil-aware.
#[derive(Clone, PartialEq)]
pub struct JsonMap<T> {
    map: BTreeMap<String, T>,
    nil: bool,
}

impl<T> Default for JsonMap<T> {
    fn default() -> Self {
        Self {
            map: BTreeMap::new(),
            nil: true,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for JsonMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nil {
            f.write_str("nil")
        } else {
            self.map.fmt(f)
        }
    }
}

impl<T> JsonMap<T> {
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
    /// Inserts `k = v`, making a nil map non-nil.
    pub fn insert(&mut self, k: impl Into<String>, v: T) {
        self.nil = false;
        self.map.insert(k.into(), v);
    }
}

impl<T> FromIterator<(String, T)> for JsonMap<T> {
    fn from_iter<I: IntoIterator<Item = (String, T)>>(iter: I) -> Self {
        Self::from_map(iter.into_iter().collect())
    }
}

impl<T> Deref for JsonMap<T> {
    type Target = BTreeMap<String, T>;
    fn deref(&self) -> &BTreeMap<String, T> {
        &self.map
    }
}
impl<T> DerefMut for JsonMap<T> {
    fn deref_mut(&mut self) -> &mut BTreeMap<String, T> {
        self.nil = false;
        &mut self.map
    }
}

impl<T: JsonDecode + Default> JsonDecode for JsonMap<T> {
    fn type_name() -> String {
        format!("map[string]{}", T::type_name())
    }
    fn json_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match v {
            Value::Null => *self = Self::nil(),
            Value::Object(m) => {
                self.nil = false;
                d.nested(|d| {
                    for (k, item) in m {
                        let mut t = T::default();
                        t.json_decode(item, d)?;
                        self.map.insert(k.clone(), t);
                    }
                    Ok::<(), Fatal>(())
                })?;
            }
            other => d.type_error(kind(other), &Self::type_name()),
        }
        Ok(())
    }
}
impl<T: JsonEncode> JsonEncode for JsonMap<T> {
    fn json_encode(&self) -> Value {
        if self.nil {
            return Value::Null;
        }
        let mut m = Map::new();
        for (k, v) in &self.map {
            m.insert(k.clone(), v.json_encode());
        }
        Value::Object(m)
    }
    fn is_omittable(&self) -> bool {
        self.map.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Untyped values and raw JSON

/// An untyped value: decoding converts numbers to float64, and encoding
/// sorts object keys.
impl JsonDecode for Value {
    fn type_name() -> String {
        "interface {}".into()
    }
    fn json_decode(&mut self, v: &Value, d: &mut Decoder) -> DResult {
        match normalize_any(v) {
            Ok(n) => *self = n,
            Err(lit) => d.type_error(&format!("number {lit}"), "float64"),
        }
        Ok(())
    }
}
impl JsonEncode for Value {
    fn json_encode(&self) -> Value {
        sort_keys(self)
    }
    fn is_omittable(&self) -> bool {
        self.is_null()
    }
}

/// Normalizes an untyped JSON value: every number becomes a float64.
pub fn normalize_any(v: &Value) -> Result<Value, String> {
    Ok(match v {
        Value::Number(n) => {
            let lit = n.to_string();
            match lit.parse::<f64>() {
                Ok(f) if f.is_finite() => float_number(f),
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

/// Recursively sorts object keys (maps are encoded with sorted keys).
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

/// A raw JSON value: kept verbatim (nil when absent).
#[derive(Clone, Default, PartialEq, Debug)]
pub struct RawMessage(pub Option<Value>);

impl JsonDecode for RawMessage {
    fn type_name() -> String {
        "json.RawMessage".into()
    }
    fn json_decode(&mut self, v: &Value, _d: &mut Decoder) -> DResult {
        self.0 = Some(v.clone());
        Ok(())
    }
}
impl JsonEncode for RawMessage {
    fn json_encode(&self) -> Value {
        self.0.clone().unwrap_or(Value::Null)
    }
    fn is_omittable(&self) -> bool {
        self.0.is_none()
    }
}

// ---------------------------------------------------------------------------
// Timestamps

impl JsonDecode for Timestamp {
    fn type_name() -> String {
        "time.Time".into()
    }
    fn json_decode(&mut self, v: &Value, _d: &mut Decoder) -> DResult {
        match v {
            Value::Null => Ok(()),
            Value::String(s) => {
                *self = Timestamp::parse_rfc3339(s).map_err(Fatal)?;
                Ok(())
            }
            _ => Err(Fatal(
                "Time.UnmarshalJSON: input is not a JSON string".into(),
            )),
        }
    }
}
impl JsonEncode for Timestamp {
    fn json_encode(&self) -> Value {
        Value::String(self.format_rfc3339_nano())
    }
}

// ---------------------------------------------------------------------------
// Structs

/// Matches an object key against a field's JSON name: an exact pass
/// first, then an ASCII case-insensitive pass.
#[doc(hidden)]
pub fn key_matches(key: &str, json: &str, exact_pass: bool) -> bool {
    if exact_pass {
        key == json
    } else {
        key.eq_ignore_ascii_case(json)
    }
}

/// Struct-level decoding hooks used by [`json_struct!`](crate::json_struct),
/// including promotion of embedded (`flatten`) struct fields.
pub trait JsonStruct {
    /// Tries to decode object member `k` into a field. `short` is the
    /// struct name reported in errors (the outermost struct for
    /// promoted fields). Returns whether a field matched.
    fn decode_member(
        &mut self,
        short: &'static str,
        k: &str,
        v: &Value,
        d: &mut Decoder,
        exact: bool,
    ) -> Result<bool, Fatal>;

    /// Writes this struct's fields into `m` in declaration order.
    fn encode_into(&self, m: &mut Map<String, Value>);
}

/// Defines a struct that decodes and encodes with the wire-format rules.
///
/// ```ignore
/// json_struct! {
///     /// Doc comment.
///     pub struct AppAttributes ("api.AppAttributes", "AppAttributes") {
///         pub name: String => "name", omitempty;
///         pub file_size: i64 => "fileSize" as "int64";
///         pub created: Option<Timestamp> => "createdDate", omitempty;
///         pub base: OtherStruct => "", flatten;   // embedded struct
///     }
/// }
/// ```
///
/// The first string is the full type name (used when a non-object is
/// decoded into the struct at top level); the second is the short struct
/// name used in field error messages (`""` for anonymous structs).
/// `as "int64"` overrides the type name reported for a scalar field.
/// `flatten` marks an embedded struct whose fields are
/// promoted into this struct's JSON object.
#[macro_export]
macro_rules! json_struct {
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

        impl $crate::wire::json::JsonStruct for $name {
            #[allow(unused_variables)]
            fn decode_member(
                &mut self,
                short: &'static str,
                k: &str,
                v: &::serde_json::Value,
                d: &mut $crate::wire::json::Decoder,
                exact: bool,
            ) -> Result<bool, $crate::wire::json::Fatal> {
                $(
                    $crate::json_struct!(@decode self, short, k, v, d, exact, $field, $json, [$($ovr)?] $(, $flag)*);
                )*
                Ok(false)
            }

            #[allow(unused_variables)]
            fn encode_into(&self, m: &mut ::serde_json::Map<String, ::serde_json::Value>) {
                $(
                    $crate::json_struct!(@encode m, self, $field, $json $(, $flag)*);
                )*
            }
        }

        impl $crate::wire::json::JsonDecode for $name {
            fn type_name() -> String { ($gotype).to_string() }
            fn json_decode(
                &mut self,
                v: &::serde_json::Value,
                d: &mut $crate::wire::json::Decoder,
            ) -> $crate::wire::json::DResult {
                match v {
                    ::serde_json::Value::Null => Ok(()),
                    ::serde_json::Value::Object(m) => {
                        for (k, val) in m {
                            for exact in [true, false] {
                                if $crate::wire::json::JsonStruct::decode_member(self, $short, k, val, d, exact)? {
                                    break;
                                }
                            }
                        }
                        Ok(())
                    }
                    other => {
                        d.type_error($crate::wire::json::kind(other), &<Self as $crate::wire::json::JsonDecode>::type_name());
                        Ok(())
                    }
                }
            }
        }

        impl $crate::wire::json::JsonEncode for $name {
            fn json_encode(&self) -> ::serde_json::Value {
                let mut m = ::serde_json::Map::new();
                $crate::wire::json::JsonStruct::encode_into(self, &mut m);
                ::serde_json::Value::Object(m)
            }
        }
    };

    (@decode $s:ident, $short:ident, $k:ident, $v:ident, $d:ident, $exact:ident, $field:ident, $json:literal, [$($ovr:literal)?] , flatten) => {
        $d.push_path($json);
        let matched = $crate::wire::json::JsonStruct::decode_member(&mut $s.$field, $short, $k, $v, $d, $exact);
        $d.pop_path();
        if matched? {
            return Ok(true);
        }
    };
    (@decode $s:ident, $short:ident, $k:ident, $v:ident, $d:ident, $exact:ident, $field:ident, $json:literal, [$($ovr:literal)?] $(, $flag:ident)*) => {
        if $crate::wire::json::key_matches($k, $json, $exact) {
            let ovr: Option<&'static str> = None $(.or(Some($ovr)))?;
            $d.field($short, $json, ovr, &mut $s.$field, $v)?;
            return Ok(true);
        }
    };

    (@encode $m:ident, $s:ident, $field:ident, $json:literal, flatten) => {
        $crate::wire::json::JsonStruct::encode_into(&$s.$field, $m);
    };
    (@encode $m:ident, $s:ident, $field:ident, $json:literal) => {
        $m.insert($json.to_string(), $crate::wire::json::JsonEncode::json_encode(&$s.$field));
    };
    (@encode $m:ident, $s:ident, $field:ident, $json:literal, omitempty) => {
        if !$crate::wire::json::JsonEncode::is_omittable(&$s.$field) {
            $m.insert($json.to_string(), $crate::wire::json::JsonEncode::json_encode(&$s.$field));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::time::Timestamp;

    json_struct! {
        struct Inner ("main.Inner", "Inner") {
            n: i64 => "n";
        }
    }
    json_struct! {
        struct Named ("main.Named", "Named") {
            s: String => "s";
            i: i64 => "i";
            i64v: i64 => "i64" as "int64";
            f: f64 => "f";
            b: bool => "b";
            p: Option<i64> => "p";
            l: JsonSlice<String> => "l";
            m: JsonMap<JsonSlice<String>> => "m";
            inner: Inner => "in";
            ins: JsonSlice<Inner> => "ins";
            t: Option<Timestamp> => "t";
            tt: Timestamp => "tt";
            a: Value => "a";
            bs: Base64Bytes => "bs";
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
    fn decode_error_messages() {
        let cases = [
            (
                r#"{"s":1}"#,
                "json: cannot unmarshal number into Go struct field Named.s of type string",
            ),
            (
                r#"{"i":"x"}"#,
                "json: cannot unmarshal string into Go struct field Named.i of type int",
            ),
            (
                r#"{"i":1.5}"#,
                "json: cannot unmarshal number 1.5 into Go struct field Named.i of type int",
            ),
            (
                r#"{"i":5.0}"#,
                "json: cannot unmarshal number 5.0 into Go struct field Named.i of type int",
            ),
            // serde_json normalises exponent literals, so `1e30` is reported as "number 1e+30".
            (
                r#"{"i":1e30}"#,
                "json: cannot unmarshal number 1e+30 into Go struct field Named.i of type int",
            ),
            (
                r#"{"i64":"x"}"#,
                "json: cannot unmarshal string into Go struct field Named.i64 of type int64",
            ),
            (
                r#"{"f":"x"}"#,
                "json: cannot unmarshal string into Go struct field Named.f of type float64",
            ),
            (
                r#"{"b":1}"#,
                "json: cannot unmarshal number into Go struct field Named.b of type bool",
            ),
            (
                r#"{"p":"x"}"#,
                "json: cannot unmarshal string into Go struct field Named.p of type int",
            ),
            (
                r#"{"l":"x"}"#,
                "json: cannot unmarshal string into Go struct field Named.l of type []string",
            ),
            (
                r#"{"l":[1]}"#,
                "json: cannot unmarshal number into Go struct field Named.l of type string",
            ),
            (
                r#"{"m":{"k":"v"}}"#,
                "json: cannot unmarshal string into Go struct field Named.m of type []string",
            ),
            (
                r#"{"m":{"k":[1]}}"#,
                "json: cannot unmarshal number into Go struct field Named.m of type string",
            ),
            (
                r#"{"m":[]}"#,
                "json: cannot unmarshal array into Go struct field Named.m of type map[string][]string",
            ),
            (
                r#"{"in":{"n":"x"}}"#,
                "json: cannot unmarshal string into Go struct field Inner.in.n of type int",
            ),
            (
                r#"{"in":5}"#,
                "json: cannot unmarshal number into Go struct field Named.in of type main.Inner",
            ),
            (
                r#"{"ins":[{"n":"x"}]}"#,
                "json: cannot unmarshal string into Go struct field Inner.ins.n of type int",
            ),
            (
                r#"{"t":"bad"}"#,
                r#"parsing time "bad" as "2006-01-02T15:04:05Z07:00": cannot parse "bad" as "2006""#,
            ),
            (
                r#"{"t":5}"#,
                "Time.UnmarshalJSON: input is not a JSON string",
            ),
            (
                r#"{"tt":"2024-01-01"}"#,
                r#"parsing time "2024-01-01" as "2006-01-02T15:04:05Z07:00": cannot parse "" as "T""#,
            ),
            (
                r#"{"t":"2024-01-01T10:00:00+0000"}"#,
                r#"parsing time "2024-01-01T10:00:00+0000" as "2006-01-02T15:04:05Z07:00": cannot parse "+0000" as "Z07:00""#,
            ),
            (r#"{"bs":"!!"}"#, "illegal base64 data at input byte 0"),
            (
                r#"{"bs":5}"#,
                "json: cannot unmarshal number into Go struct field Named.bs of type []uint8",
            ),
            (
                "[1]",
                "json: cannot unmarshal array into Go value of type main.Named",
            ),
            (
                r#""x""#,
                "json: cannot unmarshal string into Go value of type main.Named",
            ),
            ("", "unexpected end of JSON input"),
            (
                "nul",
                "invalid character ' ' in literal null (expecting 'l')",
            ),
            (
                "1.",
                "invalid character ' ' after decimal point in numeric literal",
            ),
            ("[1,", "unexpected end of JSON input"),
            ("-", "invalid character ' ' in numeric literal"),
            (
                "tr",
                "invalid character ' ' in literal true (expecting 'u')",
            ),
            ("\"\\", "invalid character ' ' in string escape code"),
            ("{\"a\"", "unexpected end of JSON input"),
            (
                "\u{1}",
                "invalid character '\\x01' looking for beginning of value",
            ),
            ("\"ab", "unexpected end of JSON input"),
            (
                "not json",
                "invalid character 'o' in literal null (expecting 'u')",
            ),
            (
                "{nope",
                "invalid character 'n' looking for beginning of object key string",
            ),
            ("[1 2]", "invalid character '2' after array element"),
            (r#"{"a" 1}"#, "invalid character '1' after object key"),
            (
                r#"{"a":1 "b"}"#,
                "invalid character '\"' after object key:value pair",
            ),
            ("01", "invalid character '1' after top-level value"),
            ("-x", "invalid character 'x' in numeric literal"),
            (
                "1.x",
                "invalid character 'x' after decimal point in numeric literal",
            ),
            (
                "1ex",
                "invalid character 'x' in exponent of numeric literal",
            ),
            (r#""\q""#, "invalid character 'q' in string escape code"),
            (
                r#""\u12x4""#,
                "invalid character 'x' in \\u hexadecimal character escape",
            ),
            ("\"a\tb\"", "invalid character '\\t' in string literal"),
            (
                r#"{"i":"x","s":2}"#,
                "json: cannot unmarshal string into Go struct field Named.i of type int",
            ),
            (r#"{"S":"case"}"#, "<nil>"),
        ];
        for (js, want) in cases {
            assert_eq!(err(js), want, "input {js}");
        }
    }

    #[test]
    fn embedded_fields_are_promoted() {
        json_struct! {
            struct Base ("tools.Base", "Base") {
                limit: i64 => "limit";
                cursor: String => "cursor";
            }
        }
        json_struct! {
            struct P ("struct { tools.Base; AppID string \"json:\\\"app_id\\\"\" }", "") {
                base: Base => "Base", flatten;
                app_id: String => "app_id";
            }
        }
        let mut p = P::default();
        unmarshal(br#"{"LIMIT":3,"app_id":"a","cursor":"c"}"#, &mut p).unwrap();
        assert_eq!(
            (p.base.limit, p.base.cursor.as_str(), p.app_id.as_str()),
            (3, "c", "a")
        );
        let mut p = P::default();
        assert_eq!(
            unmarshal(br#"{"limit":"x"}"#, &mut p).unwrap_err(),
            "json: cannot unmarshal string into Go struct field .Base.limit of type int"
        );
        assert_eq!(
            String::from_utf8(marshal(&p)).unwrap(),
            r#"{"limit":0,"cursor":"","app_id":""}"#
        );
    }

    #[test]
    fn float_formatting() {
        let cases = [
            (0.0, "0"),
            (1.0, "1"),
            (1.5, "1.5"),
            (100.0, "100"),
            (1234567.0, "1234567"),
            (123456.7, "123456.7"),
            (1e21, "1e+21"),
            (1e20, "100000000000000000000"),
            (0.0001, "0.0001"),
            (0.00001, "0.00001"),
            (-2.5e-8, "-2.5e-8"),
            (100000000.0, "100000000"),
        ];
        for (f, want) in cases {
            assert_eq!(float_json(f), want);
        }
    }

    #[test]
    fn encode_semantics() {
        json_struct! {
            struct E ("main.E", "E") {
                z: Timestamp => "z", omitempty;
                s: JsonSlice<String> => "s";
                s2: JsonSlice<String> => "s2", omitempty;
                m: JsonMap<i64> => "m";
                a: Value => "a";
                p: Option<bool> => "p", omitempty;
                h: String => "h";
            }
        }
        let e = E {
            h: "<a&b>".into(),
            ..Default::default()
        };
        assert_eq!(
            String::from_utf8(marshal(&e)).unwrap(),
            r#"{"z":"0001-01-01T00:00:00Z","s":null,"m":null,"a":null,"h":"\u003ca\u0026b\u003e"}"#
        );
        let mut any = Value::Null;
        unmarshal(
            br#"{"b":1,"a":[1.50,12345678901234567890,"x",null,true],"c":{"z":1,"y":2}}"#,
            &mut any,
        )
        .unwrap();
        assert_eq!(
            String::from_utf8(marshal(&any)).unwrap(),
            r#"{"a":[1.5,12345678901234567000,"x",null,true],"b":1,"c":{"y":2,"z":1}}"#
        );
    }
}
