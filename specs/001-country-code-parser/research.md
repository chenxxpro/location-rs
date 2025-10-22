# Research: Country Code Parser Library

**Feature**: Country Code Parser Library  
**Created**: 2025-10-22  
**Purpose**: Resolve technical unknowns and establish implementation approach

## Research Tasks

### Task 1: isocountry Crate Capabilities
**Research**: "isocountry crate capabilities for alpha2, alpha3 codes and language support"

**Findings**:
- isocountry crate提供完整的ISO 3166-1国家代码支持
- 支持alpha-2 (2字母代码如US、CN)、alpha-3 (3字母代码如USA、CHN)
- 包含国家名称的多语言支持，但中文支持需要验证
- 提供CountryCode枚举类型和转换方法

**Decision**: 使用isocountry作为主要国家代码数据源
**Rationale**: 标准化、维护良好、功能完整
**Alternatives considered**: 手动维护国家代码数据 - 维护成本高，容易出错

### Task 2: Embedded JSON Configuration Pattern
**Research**: "Rust embedded JSON configuration best practices for multi-language support"

**Findings**:
- Rust标准做法：使用`include_str!`宏嵌入配置文件
- 配置文件放在`resources/`目录，编译时嵌入二进制
- JSON格式适合多语言数据，结构清晰易维护
- 需要处理简体中文和繁体中文的编码问题

**Decision**: 使用embedded JSON配置文件管理多语言国家地区数据
**Rationale**: 编译时嵌入，运行时无文件依赖；JSON易于维护和扩展
**Alternatives considered**:
- 硬编码在代码中 - 难以维护多语言数据
- 运行时读取外部文件 - 增加部署复杂度

### Task 3: Multi-language Text Parsing Strategy
**Research**: "Text parsing strategies for Simplified and Traditional Chinese in Rust"

**Findings**:
- Rust的字符串处理原生支持UTF-8，完美支持中文
- 需要处理中文分词和模式匹配的复杂性
- 正则表达式可以用于模式匹配，但中文分词需要特殊处理
- 考虑使用简单的关键词匹配而非复杂的分词

**Decision**: 采用关键词匹配策略，维护简体中文和繁体中文的国家名称映射
**Rationale**: 实现简单，性能好，适合特定领域的文本解析
**Alternatives considered**:
- 使用中文分词库 - 增加依赖，过度复杂
- 机器学习方法 - 不必要，过度工程化

### Task 4: Error Handling Pattern for Rust Libraries
**Research**: "Rust library error handling best practices with anyhow crate"

**Findings**:
- anyhow crate提供便捷的错误处理机制
- 适合库开发，错误类型可以轻松转换为anyhow::Error
- 配合thiserror crate可以定义清晰的错误类型
- 需要定义明确的错误变体（解析失败、配置错误等）

**Decision**: 使用anyhow + thiserror组合进行错误处理
**Rationale**: 类型安全，易于使用，符合Rust最佳实践
**Alternatives considered**:
- 标准库Result - 错误处理不够便捷
- 自定义错误类型 - 实现复杂

### Task 5: Performance Optimization for Text Parsing
**Research**: "Performance optimization techniques for text parsing in Rust"

**Findings**:
- 使用HashMap进行快速关键词查找
- 避免不必要的字符串分配和复制
- 使用`&str`切片而非String克隆
- 预编译正则表达式模式
- 考虑使用Aho-Corasick算法进行多模式匹配

**Decision**: 采用HashMap + 简单模式匹配的组合策略
**Rationale**: 平衡性能和实现复杂度，满足<1ms性能目标
**Alternatives considered**:
- 纯正则表达式 - 性能可能较差
- 复杂算法 - 实现复杂，维护成本高

## Technical Decisions Summary

### Core Dependencies
- **isocountry**: 国家代码标准数据源
- **serde**: JSON配置文件的序列化/反序列化
- **anyhow + thiserror**: 错误处理
- **regex**: 可选，用于复杂模式匹配

### Configuration Approach
- **Format**: JSON配置文件
- **Location**: `resources/countries.json`
- **Embedding**: 编译时使用`include_str!`嵌入
- **Structure**: 支持简体中文和繁体中文的国家名称映射

### Parsing Strategy
- **Primary**: ISO代码直接匹配（alpha-2, alpha-3）
- **Secondary**: 关键词匹配（中文国家名称）
- **Fallback**: 组合模式识别
- **Error Handling**: 明确的错误类型和消息

### Performance Targets
- **Average Parse Time**: <1ms
- **Memory**: 最小化分配，使用引用
- **Concurrency**: 线程安全设计

## Open Questions Resolved

所有技术上下文中的未知问题已通过研究解决：

1. ✅ **Language/Version**: Rust 1.75+ 确认
2. ✅ **Primary Dependencies**: isocountry, serde, anyhow, thiserror 确认
3. ✅ **Storage**: Embedded JSON配置文件方案确定
4. ✅ **Testing**: cargo test + 性能基准测试方案
5. ✅ **Performance Goals**: <1ms目标通过优化策略可实现

## Next Steps

Phase 0研究完成，所有技术决策已确定。准备进入Phase 1设计阶段。