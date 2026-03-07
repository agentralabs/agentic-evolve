//! .aevolve binary format (placeholder for future implementation).

use crate::types::error::EvolveResult;
use crate::types::pattern::Pattern;

/// Magic bytes for .aevolve files.
pub const MAGIC: &[u8; 4] = b"AEVL";

/// Current format version.
pub const FORMAT_VERSION: u16 = 1;

/// Write patterns in .aevolve format.
pub fn write_patterns(patterns: &[&Pattern], output: &mut Vec<u8>) -> EvolveResult<()> {
    output.extend_from_slice(MAGIC);
    output.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    let count = patterns.len() as u32;
    output.extend_from_slice(&count.to_le_bytes());
    for pattern in patterns {
        let json = serde_json::to_vec(pattern)?;
        let len = json.len() as u32;
        output.extend_from_slice(&len.to_le_bytes());
        output.extend_from_slice(&json);
    }
    Ok(())
}

/// Read patterns from .aevolve format.
pub fn read_patterns(data: &[u8]) -> EvolveResult<Vec<Pattern>> {
    use crate::types::error::EvolveError;
    if data.len() < 8 {
        return Err(EvolveError::StorageError("File too small".to_string()));
    }
    if &data[0..4] != MAGIC {
        return Err(EvolveError::StorageError("Invalid magic bytes".to_string()));
    }
    let _version = u16::from_le_bytes([data[4], data[5]]);
    let count = u32::from_le_bytes([data[6], data[7], data[8], data[9]]) as usize;
    let mut patterns = Vec::with_capacity(count);
    let mut offset = 10;
    for _ in 0..count {
        if offset + 4 > data.len() {
            return Err(EvolveError::StorageError("Unexpected end of file".to_string()));
        }
        let len = u32::from_le_bytes([
            data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
        ]) as usize;
        offset += 4;
        if offset + len > data.len() {
            return Err(EvolveError::StorageError("Unexpected end of file".to_string()));
        }
        let pattern: Pattern = serde_json::from_slice(&data[offset..offset + len])?;
        patterns.push(pattern);
        offset += len;
    }
    Ok(patterns)
}
