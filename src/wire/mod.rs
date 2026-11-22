//! Compatibility layer reproducing Go standard-library semantics that the
//! server's wire behaviour depends on (JSON, formatting, URLs, time).

pub mod fmt;
pub mod json;
pub mod os;
pub mod time;
pub mod url;

pub use json::{GoBytes, GoDecode, GoEncode, GoMap, GoSlice, RawMessage};
pub use time::GoTime;
