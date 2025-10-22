<!-- 
Sync Impact Report:
- Version change: 1.0.0 → 1.1.0 (MINOR: Added new principles and governance sections)
- Modified principles: All principles replaced with new core principles
- Added sections: Development Workflow, Quality Gates
- Removed sections: None (template structure preserved)
- Templates requiring updates:
  ✅ .specify/templates/plan-template.md - Constitution Check section updated
  ✅ .specify/templates/spec-template.md - Testing requirements aligned
  ✅ .specify/templates/tasks-template.md - Task categorization updated
- Follow-up TODOs: RATIFICATION_DATE needs confirmation
-->

# location-rs Constitution

## Core Principles

### I. 代码质量优先 (Code Quality First)
所有代码必须遵循最高质量标准，包括但不限于：清晰的命名规范、适当的注释、一致的代码风格、消除重复代码、遵循单一职责原则。代码审查必须严格检查质量指标，任何质量妥协都需要充分的技术论证。

**Rationale**: 高质量的代码是长期可维护性和团队协作的基础，能够显著降低技术债务和缺陷率。

### II. 反抽象原则 (Anti-Abstraction Principle)
避免过早和不必要的抽象。只有当抽象能够解决明确的重复问题时才引入。每个抽象必须证明其价值，而不是基于"可能"的未来需求。

**Rationale**: 过度抽象会增加复杂性，降低代码可读性，并可能导致错误的架构决策。

### III. 简洁优先 (Simplicity First)
在满足功能需求的前提下，选择最简单、最直接的实现方案。复杂的解决方案必须有明确的性能或功能优势证明其必要性。

**Rationale**: 简洁的代码更容易理解、测试和维护，能够提高开发效率和系统可靠性。

### IV. 测试先行 (Test-First Development)
所有新功能开发必须采用测试驱动开发(TDD)方法：先编写测试用例，确保测试失败，然后实现功能使测试通过。测试覆盖率必须达到项目设定的标准。

**Rationale**: TDD确保代码的正确性，提供即时反馈，并作为活文档说明代码的预期行为。

### V. 测试标准 (Testing Standards)
测试必须遵循明确的标准：单元测试关注单一功能点，集成测试验证模块间协作，端到端测试验证完整业务流程。测试必须独立、可重复、快速执行。

**Rationale**: 标准化的测试方法确保测试的有效性和可维护性，为持续集成提供可靠基础。

### VI. 性能要求 (Performance Requirements)
所有代码实现必须考虑性能影响。关键路径的性能必须进行基准测试和监控。性能优化必须有数据支撑，避免过早优化。

**Rationale**: 性能是用户体验和系统可扩展性的关键因素，需要在开发早期考虑。

## Development Workflow

### 代码审查流程
所有代码变更必须经过至少一名团队成员的代码审查。审查重点包括：
- 是否符合核心原则
- 测试覆盖率和质量
- 代码可读性和可维护性
- 性能影响评估

### 质量门禁
代码合并前必须通过：
- 所有测试用例通过
- 代码静态分析无严重问题
- 性能基准测试符合要求
- 安全扫描无高危漏洞

## Governance

### 宪法修订流程
任何对宪法的修改必须：
1. 提出书面修改建议，说明修改理由和影响
2. 经过团队讨论和投票
3. 获得至少2/3成员同意
4. 更新相关模板和文档

### 合规性要求
所有项目活动必须遵守本宪法规定。违反原则的行为需要记录并说明原因。宪法是项目开发的最高指导文件。

**Version**: 1.1.0 | **Ratified**: TODO(RATIFICATION_DATE): 需要确认项目启动日期 | **Last Amended**: 2025-10-22