//! The server's wire-format rules: JSON decoding and encoding, value
//! formatting in tool output, URL query encoding, RFC 3339 timestamps and
//! file-error wording. Clients depend on this output staying stable.

pub mod fmt;
pub mod json;
pub mod os;
pub mod time;
pub mod url;

pub use json::{Base64Bytes, JsonDecode, JsonEncode, JsonMap, JsonSlice, RawMessage};
pub use time::Timestamp;
