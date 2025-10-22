# 贡献指南

感谢您对 location-rs 项目的关注！我们非常欢迎社区贡献。在提交您的贡献之前，请花一点时间阅读以下指南。

## 行为准则

参与本项目的所有贡献者都应表现出尊重和友善。请避免任何形式的骚扰、歧视或不当行为。

## 贡献方式

### 报告问题

如果您发现了 bug 或有新功能的想法，请在 GitHub 上创建一个新的 issue。请确保：

1. 使用清晰描述性的标题
2. 提供详细的步骤来重现问题
3. 包含您的环境信息（操作系统、Rust 版本等）
4. 尽可能提供最小的复现示例

### 代码贡献

1. **Fork 仓库**
   - 点击 GitHub 上的 "Fork" 按钮创建您自己的仓库副本

2. **克隆仓库**
   ```bash
   git clone https://github.com/YOUR_USERNAME/location-rs.git
   cd location-rs
   ```

3. **创建分支**
   ```bash
   git checkout -b feature-or-fix-branch
   ```

4. **安装依赖**
   ```bash
   cargo build
   ```

5. **进行更改**
   - 确保您的代码遵循 Rust 的代码风格
   - 添加适当的文档和注释
   - 为新功能添加测试用例

6. **运行测试**
   ```bash
   cargo test
   ```

7. **提交更改**
   - 使用清晰的提交消息描述您的更改
   - 遵循[Conventional Commits](https://www.conventionalcommits.org/)规范

8. **推送更改**
   ```bash
   git push origin feature-or-fix-branch
   ```

9. **创建 Pull Request**
   - 回到您的 GitHub fork 页面
   - 点击 "New Pull Request"
   - 提供详细的描述，解释您的更改和目的

## 代码风格指南

- 遵循 [Rustfmt](https://github.com/rust-lang/rustfmt) 的格式设置
- 使用 [Clippy](https://github.com/rust-lang/rust-clippy) 进行代码质量检查
- 保持代码简洁明了
- 为公共 API 提供详细的文档注释
- 遵循 Rust 社区的最佳实践

## 测试指南

- 为所有新功能添加单元测试
- 确保现有测试通过
- 使用有意义的测试用例和描述性的测试名称
- 考虑边界情况和错误处理

## 文档指南

- 更新 README.md 以反映新功能或更改
- 为公共 API 添加或更新文档注释
- 如果更改了 API，更新示例代码

## 许可证

通过向此项目贡献代码，您同意您的贡献将在 MIT 许可证下发布。