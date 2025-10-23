# Quickstart Guide: Country Code Parser Library

**Feature**: Country Code Parser Library  
**Created**: 2023-10-22  
**Purpose**: Get started with the library in 5 minutes

## Prerequisites

- Rust 1.70+ installed
- Basic familiarity with Rust and Cargo

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
location-rs = "0.1.0"
```

## Basic Usage

### 1. Simple Parsing

```rust
use location_rs::parse_country_code;

fn main() {
    // Parse country code from various text formats
    let result1 = parse_country_code("@HK Vip1");
    let result2 = parse_country_code("【SS】US1");
    let result3 = parse_country_code("V1 美国");
    
    println!("HK: {:?}", result1);
    println!("US: {:?}", result2);
    println!("CN: {:?}", result3);
}
```

### 2. Error Handling

```rust
use location_rs::{parse_country_code, ParseError};

fn process_text(text: &str) -> Result<(), ParseError> {
    match parse_country_code(text) {
        Ok(country_info) => {
            println!("Found country: {}", country_info.name_zh_cn);
            Ok(())
        }
        Err(ParseError::NotFound { text: _ }) => {
            println!("No country code found in: {}", text);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error parsing text: {}", e);
            Err(e)
        }
    }
}
```

### 3. Working with Country Information

```rust
use location_rs::{parse_country_code, CountryInfo};

fn analyze_country(text: &str) {
    if let Ok(country_info) = parse_country_code(text) {
        println!("Alpha-2 code: {}", country_info.alpha2);
        println!("Alpha-3 code: {}", country_info.alpha3);
        println!("English name: {}", country_info.name_en);
        println!("Chinese name (Simplified): {}", country_info.name_zh_cn);
        println!("Chinese name (Traditional): {}", country_info.name_zh_tw);
    }
}
```

## Advanced Usage

### Custom Configuration

```rust
use location_rs::{Parser, ParserConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create custom parser configuration
    let config = ParserConfig {
        case_sensitive: false,  // Case insensitive matching
        fuzzy_match: true,     // Enable fuzzy matching
        timeout_ms: 50,        // 50ms timeout
    };
    
    let parser = Parser::with_config(config);
    
    // Use the custom parser
    let result = parser.parse("US Node")?;
    println!("Parsed: {:?}", result);
    
    Ok(())
}
```

### Batch Processing

```rust
use location_rs::parse_country_code;

fn process_titles(titles: &[&str]) -> Vec<Option<String>> {
    titles.iter()
        .map(|title| {
            parse_country_code(title)
                .ok()
                .map(|code| code.alpha2().to_string())
        })
        .collect()
}

fn main() {
    let titles = vec!["@HK Vip1", "【SS】US1", "V1 美国", "普通标题"];
    let results = process_titles(&titles);
    
    for (title, result) in titles.iter().zip(results) {
        println!("{} -> {:?}", title, result);
    }
}
```

## Supported Text Formats

The library can parse country codes from various text patterns:

### ISO Code Patterns
- `"US Node"` → US
- `"CN Server"` → CN
- `"HK Vip"` → HK

### Chinese Name Patterns
- `"美国节点"` → US (简体中文)
- `"美國服務器"` → US (繁体中文)
- `"中国服务器"` → CN (简体中文)

### Mixed Patterns
- `"@HK Vip1"` → HK
- `"【SS】US1"` → US
- `"V1 美国"` → CN

## Error Handling Examples

```rust
use location_rs::{parse_country_code, ParseError};

fn handle_parse_result(text: &str) {
    match parse_country_code(text) {
        Ok(code) => {
            println!("Success: {}", code.alpha2());
        }
        Err(ParseError::NotFound) => {
            println!("No country code found in: {}", text);
        }
        Err(ParseError::Ambiguous(candidates)) => {
            println!("Ambiguous match for: {}", text);
            for candidate in candidates {
                println!("  Possible: {}", candidate.alpha2());
            }
        }
        Err(ParseError::InvalidInput) => {
            println!("Invalid input format: {}", text);
        }
        Err(e) => {
            println!("Unexpected error: {}", e);
        }
    }
}
```

## Performance Tips

### For High-Throughput Applications

```rust
use location_rs::Parser;

// Create parser instance once and reuse it
let parser = Parser::new();

// Process multiple texts efficiently
let texts = vec!["US1", "CN2", "JP4"];
for text in texts {
    if let Ok(code) = parser.parse(text) {
        // Fast processing
        process_country_code(code);
    }
}
```

### Memory-Efficient Processing

```rust
use location_rs::parse_country_code;

// Use string slices to avoid allocations
fn process_large_dataset<'a>(texts: &[&'a str]) -> Vec<(&'a str, Option<String>)> {
    texts.iter()
        .map(|&text| {
            let result = parse_country_code(text)
                .ok()
                .map(|code| code.alpha2().to_string());
            (text, result)
        })
        .collect()
}
```

## Testing Your Integration

### Basic Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use location_rs::{parse_country_code, CountryCode};

    #[test]
    fn test_basic_parsing() {
        assert_eq!(parse_country_code("US Node").unwrap(), CountryCode::US);
        assert_eq!(parse_country_code("中国").unwrap(), CountryCode::CN);
    }

    #[test]
    fn test_error_cases() {
        assert!(parse_country_code("普通标题").is_err());
        assert!(parse_country_code("").is_err());
    }
}
```

## Common Patterns

### Pattern 1: Extract and Process

```rust
use location_rs::parse_country_code;

fn extract_and_process(texts: &[&str]) {
    for text in texts {
        match parse_country_code(text) {
            Ok(code) => {
                println!("Processing {} for country: {}", text, code.alpha2());
                // Your processing logic here
            }
            Err(_) => {
                println!("Skipping: {}", text);
            }
        }
    }
}
```

### Pattern 2: Filter by Country

```rust
use location_rs::{parse_country_code, CountryCode};

fn filter_by_country(texts: &[&str], target_country: CountryCode) -> Vec<&str> {
    texts.iter()
        .filter(|&&text| {
            parse_country_code(text)
                .map(|code| code == target_country)
                .unwrap_or(false)
        })
        .copied()
        .collect()
}
```

## Next Steps

- Explore the full API documentation
- Check out the performance benchmarks
- Review the source code for advanced usage patterns
- Report any issues or suggest improvements