//! Go `os` error wording (`*fs.PathError`).

use std::io;

/// Go's description of an `errno` (the text after the path).
pub fn errno_text(e: &io::Error) -> String {
    match e.kind() {
        io::ErrorKind::NotFound => "no such file or directory".into(),
        io::ErrorKind::PermissionDenied => "permission denied".into(),
        io::ErrorKind::IsADirectory => "is a directory".into(),
        io::ErrorKind::NotADirectory => "not a directory".into(),
        _ => {
            // Fall back to the OS message without Rust's "(os error N)".
            let s = e.to_string();
            match s.find(" (os error") {
                Some(i) => {
                    let m = &s[..i];
                    let mut c = m.chars();
                    match c.next() {
                        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
                        None => m.to_string(),
                    }
                }
                None => s,
            }
        }
    }
}

/// Go `&fs.PathError{Op: op, Path: path, Err: e}.Error()`.
pub fn path_error(op: &str, path: &str, e: &io::Error) -> String {
    format!("{op} {path}: {}", errno_text(e))
}
