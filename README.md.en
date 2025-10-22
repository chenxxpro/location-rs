# location-rs

A high-performance Rust library for parsing country or region codes from text titles.

> **Note:** This project's implementation code and test cases are all AI-generated, developed using the latest Rust 2021 language standard.

## Features

- Supports parsing ISO 3166-1 alpha-2 and alpha-3 country codes
- Supports Simplified and Traditional Chinese country name recognition
- Multi-stage parsing algorithm for high accuracy
- Configurable parsing options (case sensitivity, fuzzy matching, timeout settings)
- Comprehensive error handling mechanism
- Zero unsafe code, fully adhering to Rust safety principles

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
location-rs = "0.1.1"
```

## Usage Examples

### Basic Usage

```rust
use location_rs::parse_country_code;

// Parse text with country code
let result = parse_country_code("@HK Vip Member");
assert!(result.is_ok());

// Parse Chinese country name
let result = parse_country_code("Chinese User Data");
assert!(result.is_ok());

// Parse ISO code
let result = parse_country_code("US-123456");
assert!(result.is_ok());
```

### Using Custom Configuration

```rust
use location_rs::{Parser, ParserConfig};

// Create custom configuration
let config = ParserConfig {
    case_sensitive: true,  // Case sensitive
    fuzzy_match: true,     // Enable fuzzy matching
    timeout_ms: 50,        // Set 50ms timeout
};

// Create parser with custom configuration
let parser = Parser::with_config(config);

// Execute parsing
let result = parser.parse("CN-SHANGHAI");
assert!(result.is_ok());
```

### Error Handling

```rust
use location_rs::parse_country_code;

// Handle possible parsing errors
let result = parse_country_code("This is text without a country code");
if let Err(error) = result {
    println!("Parsing failed: {}", error);
}
```

## Supported Formats

- ISO 3166-1 alpha-2 codes (e.g., `CN`, `US`, `JP`)
- ISO 3166-1 alpha-3 codes (e.g., `CHN`, `USA`, `JPN`)
- Simplified Chinese country names (e.g., `中国`, `美国`, `日本`)
- Traditional Chinese country names (e.g., `中國`, `美國`, `日本`)

## Parsing Algorithm

The library employs a multi-stage parsing strategy:

1. **Input Validation**: Checks text length and validity
2. **ISO Code Recognition**: Prioritizes finding standard alpha-2 and alpha-3 codes
3. **Chinese Name Matching**: Attempts to match Chinese country names
4. **Pattern Matching**: Uses fuzzy matching strategies to find possible country identifiers
5. **Timeout Protection**: Ensures the parsing process doesn't consume excessive time

## Performance

- Average parsing time < 1ms (standard input)
- Supports configurable parsing timeout to prevent performance issues
- Highly optimized string processing algorithms

## Error Types

- `InvalidInput`: Input text is empty or too long
- `NotFound`: No valid country code found in the text
- `ConfigError`: Configuration loading or parsing failed
- `Timeout`: Parsing process timed out

## Testing

This library includes comprehensive unit tests and integration tests to ensure correctness across various input scenarios.

```bash
# Run all tests
cargo test

# Run benchmark tests (optional)
cargo bench
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## Contributing

Contributions are welcome via Issues and Pull Requests! Please ensure your code passes all tests and follows Rust's code style guidelines.

## Technology Stack

- **Language**: Rust 2021
- **Main Dependencies**:
  - `serde`/`serde_json`: Configuration parsing
  - `thiserror`: Error handling
  - `criterion` (dev dependency): Performance benchmarking

---

**[中文版本 (Chinese Version)](README.md)**