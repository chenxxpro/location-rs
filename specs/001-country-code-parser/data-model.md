# Data Model: Country Code Parser Library

**Feature**: Country Code Parser Library  
**Created**: 2025-10-22  
**Purpose**: Define core entities and their relationships

## Core Entities

### CountryCode
表示国家或地区的ISO 3166-1标准代码。

**Attributes**:
- `alpha2`: String - 2字母代码 (如 "US", "CN", "HK")
- `alpha3`: String - 3字母代码 (如 "USA", "CHN", "HKG")
- `numeric`: String - 数字代码 (如 "840", "156", "344")

**Validation Rules**:
- alpha2: 必须为2个大写字母，符合ISO 3166-1标准
- alpha3: 必须为3个大写字母，符合ISO 3166-1标准
- numeric: 必须为3位数字，符合ISO 3166-1标准

**Source**: 从isocountry crate的CountryCode枚举派生

### CountryInfo
扩展的国家信息，包含多语言名称支持。

**Attributes**:
- `code`: CountryCode - 国家代码
- `name_en`: String - 英文名称
- `name_zh_cn`: String - 简体中文名称
- `name_zh_tw`: String - 繁体中文名称
- `region`: String - 所属地区

**Validation Rules**:
- 所有名称字段不能为空
- 简体中文和繁体中文名称必须正确编码
- 地区信息必须有效

### ParseResult
解析函数的返回结果。

**Variants**:
- `Success(CountryCode)` - 成功解析到国家代码
- `NotFound` - 未找到匹配的国家代码
- `Ambiguous(Vec<CountryCode>)` - 匹配到多个可能的国家代码
- `InvalidInput` - 输入文本格式无效

### Configuration
库的配置信息，从embedded JSON文件加载。

**Attributes**:
- `countries`: Vec<CountryInfo> - 国家信息列表
- `patterns`: HashMap<String, Vec<String>> - 匹配模式配置
- `settings`: ParserSettings - 解析器设置

### ParserSettings
解析器的配置设置。

**Attributes**:
- `case_sensitive`: bool - 是否区分大小写 (默认: false)
- `fuzzy_match`: bool - 是否启用模糊匹配 (默认: true)
- `timeout_ms`: u64 - 解析超时时间 (默认: 100)

## Relationships

### CountryCode ↔ CountryInfo (1:1)
每个CountryCode对应一个CountryInfo，包含多语言名称。

### Configuration → CountryInfo (1:N)
配置包含多个国家信息记录。

### ParseResult ← CountryCode (N:1)
解析结果可能包含国家代码引用。

## Data Flow

### 配置加载流程
1. 编译时嵌入`resources/countries.json`
2. 运行时解析JSON为Configuration结构
3. 构建内存中的查找表

### 解析流程
1. 输入文本预处理（清理、标准化）
2. 多阶段匹配：
   - 阶段1: ISO代码直接匹配
   - 阶段2: 关键词匹配（中文名称）
   - 阶段3: 模式匹配
3. 结果处理和错误处理

## JSON Configuration Structure

```json
{
  "version": "1.0",
  "countries": [
    {
      "alpha2": "CN",
      "alpha3": "CHN", 
      "numeric": "156",
      "name_en": "China",
      "name_zh_cn": "中国",
      "name_zh_tw": "中國",
      "region": "Asia"
    },
    {
      "alpha2": "US",
      "alpha3": "USA",
      "numeric": "840", 
      "name_en": "United States",
      "name_zh_cn": "美国",
      "name_zh_tw": "美國",
      "region": "North America"
    }
  ],
  "patterns": {
    "prefix_patterns": ["@", "【", "["],
    "suffix_patterns": ["Vip", "VIP", "节点"]
  },
  "settings": {
    "case_sensitive": false,
    "fuzzy_match": true,
    "timeout_ms": 100
  }
}
```

## Error Types

### ParseError
解析过程中可能出现的错误类型。

**Variants**:
- `ConfigLoadError` - 配置文件加载失败
- `PatternMatchError` - 模式匹配错误
- `TimeoutError` - 解析超时
- `UnsupportedEncoding` - 不支持的编码格式

## State Management

### 解析器状态
解析器是无状态的，所有配置在初始化时加载。

### 缓存策略
- 国家代码查找表：初始化时构建，只读访问
- 模式匹配器：预编译正则表达式
- 无运行时缓存，保持简单性

## Data Validation

### 输入验证
- 文本长度限制：最大1024字符
- 编码验证：必须为有效UTF-8
- 内容清理：去除多余空白字符

### 输出验证
- CountryCode必须为有效ISO代码
- 错误信息必须明确且可操作