use std::fs;
use std::io::Read;

use crate::error::{Result, TwinexError};

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

/// Read a file with the specified encoding, stripping BOM if present.
pub fn read_file_with_encoding(path: &str, encoding: Option<&str>) -> Result<String> {
    let enc = encoding.unwrap_or("UTF-8");
    let bytes = fs::read(path)?;
    let start = if bytes.len() >= 2 {
        match &bytes[..2] {
            [0xFE, 0xFF] | [0xFF, 0xFE] => 2,
            _ => 0,
        }
    } else {
        0
    };
    let content = &bytes[start..];
    match enc.to_uppercase().as_str() {
        "UTF-8" | "UTF8" => String::from_utf8(content.to_vec())
            .map_err(|e| TwinexError::Format(format!("Invalid UTF-8: {}", e))),
        "UTF-16BE" | "UTF16BE" => {
            let u16: Vec<u16> = content
                .chunks(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            String::from_utf16(&u16).map_err(|_| TwinexError::Format("Invalid UTF-16".into()))
        }
        "UTF-16LE" | "UTF16LE" => {
            let u16: Vec<u16> = content
                .chunks(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            String::from_utf16(&u16).map_err(|_| TwinexError::Format("Invalid UTF-16".into()))
        }
        _ => String::from_utf8(content.to_vec())
            .map_err(|e| TwinexError::Format(format!("Invalid encoding: {}", e))),
    }
}

/// Write a file with the specified encoding, including BOM for UTF-16 variants.
pub fn write_file(path: &str, content: &str, encoding: Option<&str>) -> Result<()> {
    let enc = encoding.unwrap_or("UTF-8");
    match enc.to_uppercase().as_str() {
        "UTF-8" | "UTF8" => Ok(fs::write(path, content)?),
        "UTF-16BE" | "UTF16BE" => {
            let u16: Vec<u16> = content.encode_utf16().collect();
            let mut bytes = vec![0xFE, 0xFF];
            for c in &u16 {
                bytes.extend_from_slice(&c.to_be_bytes());
            }
            Ok(fs::write(path, bytes)?)
        }
        "UTF-16LE" | "UTF16LE" => {
            let u16: Vec<u16> = content.encode_utf16().collect();
            let mut bytes = vec![0xFF, 0xFE];
            for c in &u16 {
                bytes.extend_from_slice(&c.to_le_bytes());
            }
            Ok(fs::write(path, bytes)?)
        }
        _ => Ok(fs::write(path, content)?),
    }
}
