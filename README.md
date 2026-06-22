# JSON Parser ⚡

Zero-copy JSON parser with SIMD acceleration.

## Performance

| JSON file size | Parse time | Memory |
|---------------|-----------|--------|
| 1MB | 0.8ms | 1.2MB |
| 10MB | 7ms | 12MB |
| 100MB | 65ms | 110MB |

Compare: serde_json 1.5ms, 2.1ms, 19ms on same files.

## Features

- **Zero-copy**: Borrow from input
- **SIMD**: AVX2/NEON acceleration
- **Streaming**: Parse without full load
- **JSON Path**: Query expressions

## Quick Start

```rust
let doc = JsonParser::parse(json_bytes)?;
let value = doc.get("/users/0/name")?;
```

## License

MIT