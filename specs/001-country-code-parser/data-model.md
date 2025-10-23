# Data Model: Country Code Parser Library

**Feature**: Country Code Parser Library  
**Created**: 2023-10-22  
**Purpose**: Define and document the data structures used in the library

## Overview

本文档描述了国家代码解析器库的数据模型，包括核心实体、它们之间的关系以及数据流。

## 核心实体

### 1. CountryInfo

**Description**: Represents comprehensive information about a country, including ISO codes and multilingual names.

**Properties**:

| Property | Type | Description |
|----------|------|-------------|
| `alpha2` | `String` | ISO 3166-1 alpha-2 two-letter code (e.g., "US", "CN") |
| `alpha3` | `String` | ISO 3166-1 alpha-3 three-letter code (e.g., "USA", "CHN") |
| `name_en` | `String` | English name of the country (e.g., "United States", "China") |
| `name_zh_cn` | `String` | Simplified Chinese name of the country (e.g., "美国", "中国") |
| `name_zh_tw` | `String` | Traditional Chinese name of the country (e.g., "美國", "中國") |

**Example Usage**:

```rust
use location_rs::{parse_country_code, CountryInfo};

if let Ok(country_info) = parse_country_code("CN") {
    assert_eq!(country_info.alpha2, "CN");
    assert_eq!(country_info.alpha3, "CHN");
    assert_eq!(country_info.name_en, "China");
    assert_eq!(country_info.name_zh_cn, "中国");
}
```

### CountryInfo

**用途**: 包含国家或地区的详细信息。

**属性**:
- `alpha2`: String (ISO 3166-1 alpha-2代码)
- `alpha3`: String (ISO 3166-1 alpha-3代码)
- `name_en`: String (英文名称)
- `name_zh_cn`: String (简体中文名称)
- `name_zh_tw`: String (繁体中文名称)
- `abbreviations`: Vec<String> (常见简称和别称)

**示例**:
```rust
let china_info = CountryInfo {
    alpha2: "CN".to_string(),
    alpha3: "CHN".to_string(),
    name_en: "China".to_string(),
    name_zh_cn: "中国".to_string(),
    name_zh_tw: "中國".to_string(),
    abbreviations: vec!["中国".to_string(), "中华".to_string()],
};
```

### 2. ParserConfig

**Description**: Configuration options for the country code parser.

**Properties**:

| Property | Type | Description |
|----------|------|-------------|
| `min_confidence` | `f32` | Minimum confidence threshold for name matching |
| `max_distance` | `usize` | Maximum Levenshtein distance for fuzzy matching |
| `enable_fuzzy_matching` | `bool` | Whether to enable fuzzy matching for names |
| `allow_partial_matches` | `bool` | Whether to allow partial matches in text |

**Example Usage**:

```rust
use location_rs::{Parser, ParserConfig};

let config = ParserConfig {
    min_confidence: 0.8,
    max_distance: 2,
    enable_fuzzy_matching: true,
    allow_partial_matches: false,
};

let parser = Parser::new_with_config(config);
```

### 3. Parser

**Description**: Main parser struct that handles country code detection and extraction from text.

**Methods**:

| Method | Description |
|--------|-------------|
| `new()` | Creates a new parser with default configuration |
| `new_with_config(config)` | Creates a new parser with custom configuration |
| `parse(text)` | Parses text to extract country information |

**Example Usage**:

```rust
use location_rs::{Parser, ParserConfig};

let parser = Parser::new();
let result = parser.parse("来自中国的访问者");

if let Ok(country_info) = result {
    println!("Detected country: {}", country_info.name_en);
}
```

### 4. ParseError

**Description**: Error type for country code parsing operations.

**Variants**:

| Variant | Description |
|---------|-------------|
| `NotFound { text }` | No country code or name was found in the input |
| `InvalidFormat { text }` | The input format was invalid or not recognized |
| `MultipleMatches { matches }` | Multiple possible matches were found |
| `ConfigError { message }` | Error in parser configuration |
| `Timeout` | Parsing operation timed out |

**Example Usage**:

```rust
use location_rs::{parse_country_code, ParseError};

match parse_country_code("unknown-country-code") {
    Ok(country_info) => println!("Found: {}", country_info.name_en),
    Err(ParseError::NotFound { text }) => println!("Not found: {}", text),
    Err(ParseError::InvalidFormat { text }) => println!("Invalid format: {}", text),
    Err(e) => println!("Other error: {}", e),
}
```

### 5. 实用工具函数

**Description**: 提供简单易用的顶层API函数，方便快速集成。

**Functions**:

| Function | Description |
|----------|-------------|
| `parse_country_code(text)` | 快速解析文本中的国家代码或名称，返回`Result<CountryInfo, ParseError>` |

**Example Usage**:

```rust
use location_rs::parse_country_code;

// 解析ISO alpha-2代码
let china = parse_country_code("CN").unwrap();
assert_eq!(china.name_zh_cn, "中国");

// 解析国家中文名称
let japan = parse_country_code("日本").unwrap();
assert_eq!(japan.alpha2, "JP");

// 解析混合文本中的国家信息
let result = parse_country_code("我来自美国(USA)").unwrap();
assert_eq!(result.alpha2, "US");
```

## 数据关系

```
ParserConfig ────► Parser ────► Result<CountryInfo, ParseError>
     ▲                               │
     │                               │
     └───────── Configuration ───────┘
                      │
                      ▼
              HashMap<String, CountryInfo>
```

## 配置加载流程

1. 库从嵌入的JSON文件(`resources/countries.json`)加载配置
2. 配置被解析为`Configuration`对象
3. 国家信息被提取并存储在`HashMap`结构中以实现高效查找

## 解析流程

1. 输入文本验证（非空、不过长）
2. 文本预处理（如果`case_sensitive`为false则进行大小写标准化）
3. 按顺序尝试多种解析策略：
   - ISO代码匹配（alpha-2, alpha-3）
   - 中文名称匹配（简体、繁体）
   - 模式匹配（如果启用了模糊匹配）
4. 如果找到匹配项，返回`CountryInfo`
5. 如果未找到匹配项或发生错误，返回`ParseError`

## JSON配置结构

```json
{
  "version": "1.1",
  "countries": [
    {
      "alpha2": "CN",
      "alpha3": "CHN",
      "name_en": "China",
      "name_zh_cn": "中国",
      "name_zh_tw": "中國",
      "abbreviations": ["中国", "中华"]
    },
    // 更多国家...
  ],
  "patterns": {
    "prefix_patterns": ["@", "【", "[", "#"],
    "suffix_patterns": ["]", "】", " "]
  },
  "settings": {
    "case_sensitive": false,
    "fuzzy_match": true
  }
}
```

## 错误处理

库使用`thiserror` crate进行错误处理，为不同的错误场景提供详细的错误消息和类型。

## 数据验证

- 输入文本不能为空或仅包含空白字符
- 输入文本不能超过1024个字符
- ISO代码会根据已知模式和边界条件进行验证
- 边界字符检查确保不会将普通文本误识别为国家代码

## 性能考虑

- 国家信息在初始化时只加载一次
- 使用高效的哈希映射查找进行国家代码匹配
- 多种解析策略按效率顺序尝试

## 线程安全性

- 解析器设计为线程安全，可以在并发环境中使用
- 配置数据一旦加载就不可变
- 解析操作不会修改共享状态

## 多阶段解析策略

1. **ISO代码解析**：首先尝试直接匹配ISO 3166-1 alpha-2和alpha-3代码
2. **中文名称解析**：然后尝试匹配简体中文和繁体中文国家名称
3. **模式匹配解析**：最后使用模糊匹配策略尝试匹配各种格式的国家标识

这种多阶段策略确保了高效解析和广泛的格式支持。