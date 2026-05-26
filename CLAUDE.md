# CodeDog 项目规范

## Git 工作流程

### 分支策略

```
main (保护分支)
  │
  ├── develop (开发分支)
  │     │
  │     ├── feature/xxx (功能分支)
  │     ├── fix/xxx (修复分支)
  │     └── refactor/xxx (重构分支)
  │
  └── hotfix/xxx (紧急修复分支)
```

### 严格禁止

- **禁止直接推送到 main 分支**
- **禁止直接推送到 develop 分支**
- 所有代码变更必须通过 Pull Request 合并

### 工作流程

| 操作类型 | 分支流程 |
|---------|---------|
| 新功能 | `develop` → `feature/xxx` → PR → `develop` → PR → `main` |
| Bug修复 | `develop` → `fix/xxx` → PR → `develop` → PR → `main` |
| 紧急修复 | `main` → `hotfix/xxx` → PR → `main` |

### 分支命名规范

| 类型 | 格式 | 示例 |
|------|------|------|
| 功能 | `feature/描述` | `feature/add-json-validator` |
| 修复 | `fix/描述` | `fix/timezone-conversion` |
| 重构 | `refactor/描述` | `refactor/tui-layout` |
| 紧急修复 | `hotfix/描述` | `hotfix/critical-crash` |

### Pull Request 规范

1. **标题格式**: 遵循 Conventional Commits
   - `feat: 添加 JSON 验证功能`
   - `fix: 修复时区转换错误`
   - `refactor: 重构 TUI 布局`

2. **CI 检查**: 所有 PR 必须通过 CI 检查才能合并

3. **合并方式**: 
   - 功能分支 → develop: Squash merge
   - develop → main: Squash merge
   - hotfix → main: Squash merge

### Issue 规范

- 新功能开发前先创建 Issue
- 使用标签分类: `feature`, `bug`, `enhancement`, `documentation`
- PR 中关联 Issue: `Closes #123`

### 提交信息规范

遵循 [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>: <description>

[optional body]

Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>
```

**类型说明:**
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `test`: 测试相关
- `refactor`: 代码重构
- `style`: 代码格式
- `chore`: 构建/工具变更
- `ci`: CI/CD 配置

## AI 助手行为规范

### 开始工作前

1. 检查当前分支:
   - 如果在 `main` 分支，必须先创建 `develop` 或功能分支
   - 如果在 `develop` 分支，从它创建功能分支

2. 创建分支命令:
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feature/xxx
   ```

### 完成工作后

1. 提交代码到当前分支
2. 推送到远程
3. 创建 Pull Request
4. **不要直接合并 PR**，等待用户确认或 CI 通过

### 禁止操作

- 禁止 `git push origin main`
- 禁止 `git push origin develop`
- 禁止未经确认的 `git merge`
- 禁止强制推送 (`--force`)

## 项目结构

```
codedog-cli/
├── src/
│   ├── main.rs        # 入口点
│   ├── cli/           # CLI 命令
│   ├── tui/           # TUI 界面
│   ├── core/          # 核心功能
│   └── utils/         # 工具函数
├── tests/             # 集成测试
├── docs/              # 文档
├── .github/workflows/ # CI/CD
└── scripts/           # 打包脚本
```

## 测试规范

- 新功能必须添加单元测试
- 测试覆盖率目标: 80%+
- 运行测试: `cargo test`
- 代码格式化: `cargo fmt --all`
- 静态检查: `cargo clippy`
