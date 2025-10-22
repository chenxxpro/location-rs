# Feature Specification: Country Code Parser Library

**Feature Branch**: `001-country-code-parser`  
**Created**: 2025-10-22  
**Status**: Implemented  
**Input**: User description: "创建一个Rust库项目，实现以下功能：解析一个标题文本（参考如"@HK Vip1"、"【SS】USA1"，"V1 美国"）中的国家或者地区，返回其对应的CountryCode"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 解析包含国家代码的标题文本 (Priority: P1)

开发者需要从各种格式的标题文本中提取国家或地区代码，用于国际化应用、内容分类或数据分析。

**Why this priority**: 这是核心功能，没有这个功能库就没有价值。

**Independent Test**: 可以通过调用解析函数并验证返回的国家代码是否正确来独立测试。

**Acceptance Scenarios**:

1. **Given** 标题文本 "@HK Vip1"，**When** 调用解析函数，**Then** 返回香港的国家代码 "HK"
2. **Given** 标题文本 "【测试】USA1"，**When** 调用解析函数，**Then** 返回美国的国家代码 "US"
3. **Given** 标题文本 "V1 美国"，**When** 调用解析函数，**Then** 返回美国的国家代码 "US"
4. **Given** 标题文本 "CN Server"，**When** 调用解析函数，**Then** 返回中国的国家代码 "CN"
5. **Given** 标题文本 "【韩国-456】Server"，**When** 调用解析函数，**Then** 返回韩国的国家代码 "KR"

---

### User Story 2 - 处理无法识别的国家代码和错误情况 (Priority: P2)

当标题文本中不包含可识别的国家或地区代码时，库需要提供适当的错误处理机制，并处理各种边缘情况。

**Why this priority**: 错误处理是库的健壮性要求，确保在各种输入情况下都能正常工作。

**Independent Test**: 可以通过提供不包含国家代码的文本并验证错误处理来独立测试。

**Acceptance Scenarios**:

1. **Given** 标题文本 "普通标题"，**When** 调用解析函数，**Then** 返回NotFound错误
2. **Given** 标题文本 "12345"，**When** 调用解析函数，**Then** 返回NotFound错误
3. **Given** 空文本，**When** 调用解析函数，**Then** 返回InvalidInput错误
4. **Given** 超长文本（超过1024字符），**When** 调用解析函数，**Then** 返回InvalidInput错误
5. **Given** 解析超时的情况，**When** 调用解析函数，**Then** 返回Timeout错误

---

### User Story 3 - 支持多种国家代码格式和匹配策略 (Priority: P3)

库需要支持常见的国家代码格式和多种匹配策略，包括ISO 3166-1 alpha-2/alpha-3代码、国家名称（中文/英文）、简称和别称。

**Why this priority**: 提高库的实用性和兼容性，支持更多使用场景。

**Independent Test**: 可以通过提供不同格式的国家代码并验证解析结果来独立测试。

**Acceptance Scenarios**:

1. **Given** 标题文本包含 "CN"，**When** 调用解析函数，**Then** 返回中国的国家代码 "CN"
2. **Given** 标题文本包含 "中国"，**When** 调用解析函数，**Then** 返回中国的国家代码 "CN"
3. **Given** 标题文本包含 "United States"，**When** 调用解析函数，**Then** 返回美国的国家代码 "US"
4. **Given** 标题文本包含 "USA"，**When** 调用解析函数，**Then** 返回美国的国家代码 "US"
5. **Given** 标题文本包含 "印尼"，**When** 调用解析函数，**Then** 返回印度尼西亚的国家代码 "ID"

### Edge Cases

- **多国家代码处理**：当标题文本包含多个国家代码时，库会优先匹配优先级较高的代码（alpha-2 > alpha-3 > 中文名称 > 英文名称）
- **边界字符处理**：库会检查国家代码周围的字符，确保不会将普通文本误识别为国家代码
- **空文本处理**：对于空文本或只包含空白字符的输入，库会返回InvalidInput错误
- **超长文本处理**：对于超过1024字符的输入，库会返回InvalidInput错误
- **不区分大小写**：默认情况下，库不区分大小写进行匹配
- **模糊匹配**：默认情况下，库启用模糊匹配，可以处理部分匹配情况
- **超时处理**：库实现了超时机制，防止解析过程耗时过长

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 库必须提供一个函数，能够解析标题文本中的国家或地区代码
- **FR-002**: 库必须支持ISO 3166-1 alpha-2国家代码格式（如US、CN、HK）
- **FR-003**: 库必须支持ISO 3166-1 alpha-3国家代码格式（如USA、CHN、HKG）
- **FR-004**: 库必须支持简体中文国家名称的识别（如美国、中国、香港）
- **FR-005**: 库必须支持繁体中文国家名称的识别（如美國、中國、香港）
- **FR-006**: 库必须支持英文国家名称的识别
- **FR-007**: 库必须支持国家简称和别称的识别
- **FR-008**: 库必须正确处理无法识别国家代码的情况，返回明确的错误指示
- **FR-009**: 库必须能够处理各种文本格式（包含特殊字符、数字、中文等）
- **FR-010**: 库必须使用isocountry crate作为国家代码数据源
- **FR-011**: 库必须遵循Rust的最佳实践，提供清晰的API文档和错误处理
- **FR-012**: 库必须实现多阶段解析策略（ISO代码、中文名称、模式匹配）
- **FR-013**: 库必须实现配置化的解析选项（区分大小写、模糊匹配、超时设置）

### Key Entities

- **CountryCode**: 表示国家或地区的ISO 3166-1 alpha-2代码（来自isocountry crate）
- **CountryInfo**: 包含国家完整信息的数据结构，包括alpha2、alpha3、英文名称、中文名称等
- **ParserConfig**: 解析器配置，包含区分大小写、模糊匹配、超时设置等选项
- **TitleText**: 需要解析的输入文本，可能包含各种格式的国家信息
- **ParseError**: 解析错误类型，包含多种错误情况（NotFound、InvalidInput、Timeout等）

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 库能够正确解析至少95%的常见标题格式中的国家代码
- **SC-002**: 解析函数在标准硬件上的平均执行时间不超过1毫秒
- **SC-003**: 库的API设计清晰，开发者能够在5分钟内理解如何使用
- **SC-004**: 错误处理机制完善，所有边界情况都有明确的处理方式
- **SC-005**: 代码覆盖率至少达到90%，确保功能的可靠性
- **SC-006**: 支持250+国家和地区的识别
- **SC-007**: 支持简体中文和繁体中文标题文本解析
- **SC-008**: 支持多线程并发调用，线程安全