# 贡献指南

感谢你对 CodeDog 的关注！

## 开发环境

- Rust 1.70+
- Git

## 快速开始

```bash
# 克隆仓库
git clone https://github.com/your-username/codedog-cli.git
cd codedog-cli

# 构建项目
cargo build

# 运行测试
cargo test

# 运行
cargo run -- --help
```

## 代码规范

### 提交信息

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：

```
feat: 添加新功能
fix: 修复 bug
docs: 文档更新
test: 测试相关
refactor: 重构代码
chore: 构建/工具变更
```

### 代码风格

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 添加单元测试覆盖新功能

## 发布流程

### 创建新版本

1. 更新 `Cargo.toml` 中的版本号
2. 更新 `CHANGELOG.md`
3. 提交变更：
   ```bash
   git commit -m "chore: 发布 v0.2.0"
   ```
4. 创建标签：
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```
5. GitHub Actions 将自动构建并发布

## 项目结构

```
src/
├── main.rs        # 入口点
├── cli/           # CLI 命令
├── tui/           # TUI 界面
├── core/          # 核心功能
└── utils/         # 工具函数
```

## 添加新功能

1. 在 `src/core/` 中实现核心逻辑
2. 在 `src/cli/` 中添加 CLI 命令
3. 在 `src/tui/` 中添加 TUI 支持（可选）
4. 添加单元测试
5. 更新文档
