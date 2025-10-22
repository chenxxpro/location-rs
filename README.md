# location-rs

一个高性能的 Rust 库，用于从文本标题中解析国家或地区代码。

> **注意：** 本项目的实现代码和测试用例均由 AI 生成，采用最新的 Rust 2021 语言标准开发。

---

**[English Version](README.md.en)**

## 功能特性

- 支持解析 ISO 3166-1 alpha-2 和 alpha-3 国家代码
- 支持简体中文和繁体中文国家名称识别
- 多阶段解析算法，确保高准确率
- 可配置的解析选项（大小写敏感性、模糊匹配、超时设置）
- 完善的错误处理机制
- 零不安全代码，完全遵循 Rust 安全原则

## 安装

将以下内容添加到你的 `Cargo.toml` 文件中：

```toml
[dependencies]
location-rs = "0.1.0"
```

## 使用示例

### 基本用法

```rust
use location_rs::parse_country_code;

// 解析带有国家代码的文本
let result = parse_country_code("@HK Vip会员");
assert!(result.is_ok());

// 解析中文国家名称
let result = parse_country_code("中国用户数据");
assert!(result.is_ok());

// 解析 ISO 代码
let result = parse_country_code("US-123456");
assert!(result.is_ok());
```

### 使用自定义配置

```rust
use location_rs::{Parser, ParserConfig};

// 创建自定义配置
let config = ParserConfig {
    case_sensitive: true,  // 区分大小写
    fuzzy_match: true,     // 启用模糊匹配
    timeout_ms: 50,        // 设置50毫秒超时
};

// 使用自定义配置创建解析器
let parser = Parser::with_config(config);

// 执行解析
let result = parser.parse("CN-SHANGHAI");
assert!(result.is_ok());
```

### 处理错误

```rust
use location_rs::parse_country_code;

// 处理可能的解析错误
let result = parse_country_code("这是一段没有国家代码的文本");
if let Err(error) = result {
    println!("解析失败: {}", error);
}
```

## 支持的格式

- ISO 3166-1 alpha-2 代码（如：`CN`, `US`, `JP`）
- ISO 3166-1 alpha-3 代码（如：`CHN`, `USA`, `JPN`）
- 简体中文国家名称（如：`中国`, `美国`, `日本`）
- 繁体中文国家名称（如：`中國`, `美國`, `日本`）

## 解析算法

该库采用多阶段解析策略：

1. **输入验证**：检查文本长度和有效性
2. **ISO 代码识别**：优先查找标准的 alpha-2 和 alpha-3 代码
3. **中文名称匹配**：尝试匹配中文国家名称
4. **模式匹配**：使用模糊匹配策略寻找可能的国家标识
5. **超时保护**：确保解析过程不会占用过多时间

## 性能

- 平均解析时间 < 1ms（标准输入）
- 支持配置解析超时时间，防止性能问题
- 高度优化的字符串处理算法

## 错误类型

- `InvalidInput`：输入文本为空或过长
- `NotFound`：在文本中未找到有效的国家代码
- `ConfigError`：配置加载或解析失败
- `Timeout`：解析过程超时

## 测试

本库包含全面的单元测试和集成测试，确保在各种输入情况下的正确性。

```bash
# 运行所有测试
cargo test

# 运行基准测试（可选）
cargo bench
```

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 贡献

欢迎提交 Issues 和 Pull Requests！请确保你的代码通过所有测试并遵循 Rust 的代码风格指南。

## 技术栈

- **语言**：Rust 2021
- **主要依赖**：
  - `isocountry`：ISO 国家代码标准实现
  - `serde`/`serde_json`：配置解析
  - `thiserror`：错误处理
  - `criterion`（开发依赖）：性能基准测试

---

**[English Version](README.md.en)**