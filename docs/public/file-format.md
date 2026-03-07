---
status: stable
---

# .aevolve File Format

## Overview

The `.aevolve` binary format stores patterns with metadata, templates, and usage statistics. It is versioned for forward compatibility.

## Structure

```
+------------------+
| Magic: "AEVO"    |  4 bytes
| Version: u16     |  2 bytes
| Flags: u16       |  2 bytes
+------------------+
| Header length    |  4 bytes (u32 LE)
| Header (JSON)    |  variable
+------------------+
| Pattern count    |  4 bytes (u32 LE)
+------------------+
| Pattern 1        |
|   Length (u32 LE) |
|   Data (bincode)  |
+------------------+
| Pattern 2        |
|   ...             |
+------------------+
| Index offset     |  8 bytes (u64 LE)
| Index (bincode)  |  variable
+------------------+
| Checksum (BLAKE3)|  32 bytes
+------------------+
```

## Versioning

- Version 1: Initial format (v0.1.0)
- Format is forward-compatible: older readers skip unknown fields
- BLAKE3 checksum covers all data before the checksum

## Integrity

- BLAKE3 checksum validates file integrity on load
- Corrupted files are rejected with a clear error message
- Atomic writes prevent partial-write corruption
