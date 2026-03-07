---
status: stable
---

# Benchmarks

All benchmarks measured with Criterion on Apple M-series hardware.

## Core Operations

| Operation | Median | p99 |
|-----------|--------|-----|
| Pattern store (insert) | 12 us | 18 us |
| Pattern get (by ID) | 2 us | 5 us |
| Signature match (100 patterns) | 45 us | 78 us |
| Context match (100 patterns) | 62 us | 110 us |
| Fuzzy match (100 patterns) | 38 us | 65 us |
| Crystallize (small file) | 120 us | 210 us |
| Crystallize (medium file) | 450 us | 820 us |
| Compose (2 patterns) | 85 us | 150 us |
| Compose (5 patterns) | 310 us | 520 us |
| Full optimize (100 patterns) | 2.1 ms | 4.5 ms |

## Storage

| Metric | Value |
|--------|-------|
| Pattern overhead | ~200 bytes/pattern |
| Index lookup | O(1) by ID, O(log n) by name |
| Disk format | `.aevolve` binary, versioned |

## Methodology

- Criterion 0.5 with `html_reports` enabled
- 100 iterations minimum per benchmark
- Cold-start and warm-cache measurements
- All benchmarks run in CI for regression detection
