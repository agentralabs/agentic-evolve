---
status: stable
---

# FFI Reference

## C API

All FFI functions use C-compatible types and return error codes.

### Store Operations

```c
// Open or create a pattern store
// Returns: 0 on success, non-zero on error
int aevolve_store_open(const char *path, AevolveStore **out);

// Close a pattern store
void aevolve_store_close(AevolveStore *store);

// Insert a pattern
int aevolve_store_insert(AevolveStore *store, const char *name, const char *domain,
                         const char *lang, const char *body);

// Get a pattern by name
// Returns: JSON string (caller must free with aevolve_string_free)
char *aevolve_store_get(AevolveStore *store, const char *name);

// Search patterns
// Returns: JSON array string (caller must free with aevolve_string_free)
char *aevolve_store_search(AevolveStore *store, const char *query, int limit);

// Delete a pattern
int aevolve_store_delete(AevolveStore *store, const char *name);
```

### Memory Management

```c
// Free a string returned by aevolve functions
void aevolve_string_free(char *s);

// Get last error message
const char *aevolve_last_error(void);
```

## Python Usage

```python
import ctypes

lib = ctypes.CDLL("libagentic_evolve_ffi.so")
store = ctypes.c_void_p()
lib.aevolve_store_open(b"/tmp/test.aevolve", ctypes.byref(store))
```

## Node.js Usage

```javascript
const ffi = require('ffi-napi');
const lib = ffi.Library('libagentic_evolve_ffi', {
  'aevolve_store_open': ['int', ['string', 'pointer']],
});
```
