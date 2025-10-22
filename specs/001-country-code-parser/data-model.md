# Data Model: Country Code Parser Library

## Overview

本文档描述了国家代码解析器库的数据模型，包括核心实体、它们之间的关系以及数据流。

## 核心实体

### CountryCode

**用途**: 使用ISO 3166-1 alpha-2代码表示国家或地区。

**说明**:
- 直接使用`isocountry` crate中的`CountryCode`类型
- 表示标准的ISO 3166-1 alpha-2国家代码

**示例**:
```rust
// 通过isocountry crate获取
let china_code = isocountry::country_by_alpha2("CN").unwrap();
let usa_code = isocountry::country_by_alpha2("US").unwrap();
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

### ParserConfig

**用途**: 国家代码解析器的配置选项。

**属性**:
- `case_sensitive`: bool (是否执行区分大小写的匹配)
- `fuzzy_match`: bool (是否启用模糊匹配)
- `timeout_ms`: u64 (超时时间，以毫秒为单位)

**默认值**:
- `case_sensitive`: false (默认不区分大小写)
- `fuzzy_match`: true (默认启用模糊匹配)
- `timeout_ms`: 100 (默认超时时间为100毫秒)

**示例**:
```rust
let config = ParserConfig {
    case_sensitive: false,
    fuzzy_match: true,
    timeout_ms: 100,
};
```

### Parser

**用途**: 解析器实例，包含配置并提供解析功能。

**属性**:
- `config`: ParserConfig (解析器配置)

**方法**:
- `new()`: 使用默认配置创建解析器
- `with_config(config)`: 使用自定义配置创建解析器
- `parse(text)`: 解析文本中的国家代码

**示例**:
```rust
// 使用默认配置
let parser = Parser::new();
let result = parser.parse("@HK Vip1");

// 使用自定义配置
let custom_config = ParserConfig {
    case_sensitive: true,
    fuzzy_match: false,
    timeout_ms: 50,
};
let custom_parser = Parser::with_config(custom_config);
```

### ParseError

**用途**: 表示解析过程中可能发生的错误。

**变体**:
- `NotFound`: 输入文本中未找到国家代码
- `Ambiguous`: 找到多个可能的国家代码
- `InvalidInput`: 输入文本无效（为空、过长等）
- `Timeout`: 解析时间超过指定的超时时间
- `ConfigError`: 配置加载或解析错误

**错误处理方法**:
- `not_found(text)`: 创建未找到错误
- `ambiguous(text, candidates)`: 创建模糊匹配错误
- `invalid_input(text)`: 创建无效输入错误
- `timeout(timeout_ms)`: 创建超时错误
- `config_error(message)`: 创建配置错误

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
    "fuzzy_match": true,
    "timeout_ms": 100
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
- 超时机制防止长时间运行的解析操作

## 线程安全性

- 解析器设计为线程安全，可以在并发环境中使用
- 配置数据一旦加载就不可变
- 解析操作不会修改共享状态

## 多阶段解析策略

1. **ISO代码解析**：首先尝试直接匹配ISO 3166-1 alpha-2和alpha-3代码
2. **中文名称解析**：然后尝试匹配简体中文和繁体中文国家名称
3. **模式匹配解析**：最后使用模糊匹配策略尝试匹配各种格式的国家标识

这种多阶段策略确保了高效解析和广泛的格式支持。