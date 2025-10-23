# API Contracts: Country Code Parser Library

**Feature**: Country Code Parser Library  
**Created**: 2025-10-22  
**Purpose**: Define public API interfaces and contracts

## Public API Interface

### Core Function

#### `parse_country_code(text: &str) -> Result<CountryCode, ParseError>`
解析输入文本中的国家或地区代码。

**Parameters**:
- `text`: &str - 要解析的标题文本

**Returns**:
- `Ok(CountryCode)` - 成功解析到的国家代码
- `Err(ParseError)` - 解析失败的错误信息

**Behavior**:
- 支持多种文本格式（包含特殊字符、数字、中文等）
- 支持ISO 3166-1 alpha-2代码（如US、CN、HK）
- 支持常见国家名称识别（简体中文和繁体中文）
- 性能要求：平均执行时间 <1ms

**Examples**:
```rust
// 成功案例
assert_eq!(parse_country_code("@HK Vip1").unwrap(), CountryCode::HK);
assert_eq!(parse_country_code("【SS】US1").unwrap(), CountryCode::US);
assert_eq!(parse_country_code("V1 美国").unwrap(), CountryCode::CN);

// 错误案例
assert!(parse_country_code("普通标题").is_err());
assert!(parse_country_code("12345").is_err());
```

### Configuration API

#### `ParserConfig`
解析器配置结构，支持自定义配置。

**Fields**:
- `case_sensitive: bool` - 是否区分大小写（默认：false）
- `fuzzy_match: bool` - 是否启用模糊匹配（默认：true）
- `timeout_ms: u64` - 解析超时时间（默认：100ms）

#### `Parser::with_config(config: ParserConfig) -> Parser`
使用自定义配置创建解析器实例。

### Error Types

#### `ParseError`
解析错误枚举类型。

**Variants**:
- `NotFound` - 未找到匹配的国家代码
- `Ambiguous(Vec<CountryCode>)` - 匹配到多个可能的国家代码
- `InvalidInput` - 输入文本格式无效
- `Timeout` - 解析超时
- `ConfigError` - 配置错误

**Error Handling**:
- 所有错误都提供清晰的错误信息
- 支持错误链和上下文信息
- 错误类型可以轻松转换为anyhow::Error

## Data Types

### `CountryCode`
国家代码枚举，基于ISO 3166-1标准。

**Source**: 从isocountry crate重新导出

**Methods**:
- `alpha2() -> &str` - 获取2字母代码
- `alpha3() -> &str` - 获取3字母代码
- `name() -> &str` - 获取英文名称

### `ParseResult` (内部类型)
解析结果类型，不直接暴露给用户。

## Feature Flags

### 可选功能特性

**`serde`** - 启用序列化支持
- 允许CountryCode等类型的序列化/反序列化

**`async`** - 启用异步支持（未来扩展）
- 异步解析接口
- 批量处理支持

## Usage Patterns

### 基本用法
```rust
use location_rs::{parse_country_code, CountryCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = parse_country_code("@HK Vip1")?;
    println!("Detected country: {} ({})", code.alpha2(), code.name());
    Ok(())
}
```

### 高级用法（自定义配置）
```rust
use location_rs::{Parser, ParserConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ParserConfig {
    case_sensitive: true,
    fuzzy_match: false,
};
    
    let parser = Parser::with_config(config);
    let code = parser.parse("US Node")?;
    Ok(())
}
```

## Performance Contracts

### 响应时间
- **平均解析时间**: <1ms（标准硬件）
- **最坏情况**: <10ms（复杂文本模式）

### 内存使用
- **初始化内存**: <1MB（包含所有国家数据）
- **每次解析**: 最小化内存分配
- **无内存泄漏**: 确保正确资源清理

### 线程安全
- **线程安全**: 所有公共API都是线程安全的
- **无副作用**: 解析操作不会修改内部状态
- **可重入**: 支持并发调用

## Compatibility

### Rust版本
- **最低版本**: Rust 1.75+
- **稳定版本**: 与最新稳定版兼容

### 平台支持
- **操作系统**: Linux, macOS, Windows
- **架构**: x86_64, aarch64
- **特性**: 无特定平台依赖

## Testing Contracts

### 单元测试覆盖
- **行覆盖率**: ≥90%
- **分支覆盖率**: ≥85%
- **错误路径**: 所有错误情况都有测试

### 集成测试
- **API稳定性**: 公共API向后兼容
- **性能基准**: 定期性能测试
- **跨平台**: 多平台一致性测试

## Versioning Policy

### 语义化版本
- **主版本**: 不兼容的API更改
- **次版本**: 向后兼容的功能添加
- **修订版本**: 向后兼容的问题修复

### 弃用策略
- **弃用通知**: 提前一个主版本通知
- **迁移指南**: 提供清晰的迁移路径
- **向后兼容**: 确保平滑升级