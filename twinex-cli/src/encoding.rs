use std::fs;
use std::io::Read;

/// Detect BOM and return the encoding name
pub fn bom(path: &str) -> Option<String> {
    let mut file = fs::File::open(path).ok()?;
    let mut buf = [0u8; 2];
    if file.read_exact(&mut buf).is_err() {
        return None;
    }
    match buf {
        [0xFE, 0xFF] => Some("UTF-16BE".to_string()),
        [0xFF, 0xFE] => Some("UTF-16LE".to_string()),
        _ => None,
    }
}

pub fn has_bom(path: &str) -> bool {
    bom(path).is_some()
}

pub fn encoding_for_path(path: &str) -> String {
    bom(path).unwrap_or_else(|| "UTF-8".to_string())
}