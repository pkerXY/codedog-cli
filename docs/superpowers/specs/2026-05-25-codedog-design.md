---
name: codedog-design
description: CodeDog CLI+TUI 程序员工具箱设计文档
metadata:
  type: project
---

# CodeDog (dog) - 程序员工具箱

## 项目概述

**CodeDog** 是一个面向个人开发者的轻量级 CLI+TUI 工具箱，提供常用的实用工具功能。命令行入口为 `dog`。

### 目标用户

个人开发者，用于日常开发效率提升。

### 核心定位

实用工具类工具箱，专注数据格式化、编码转换和时间工具三大功能。

## 技术栈

- **语言**: Rust
- **CLI 框架**: clap (derive)
- **TUI 框架**: ratatui + crossterm
- **数据处理**: serde, serde_json, serde_yaml, toml
- **编码**: base64, url, md-5, sha2
- **时间**: chrono, chrono-tz

## 架构设计

### 项目结构

```
codedog-cli/
├── Cargo.toml
├── src/
│   ├── main.rs              # 入口点，命令路由
│   ├── cli/                  # CLI 模式实现
│   │   ├── mod.rs
│   │   ├── format.rs         # 格式化命令
│   │   ├── encode.rs         # 编码命令
│   │   └── time.rs           # 时间命令
│   ├── tui/                  # TUI 模式实现
│   │   ├── mod.rs
│   │   ├── app.rs            # TUI 应用主循环
│   │   ├── ui/               # UI 组件
│   │   └── widgets/          # 自定义控件
│   ├── core/                 # 核心功能实现
│   │   ├── mod.rs
│   │   ├── formatter/        # 格式化逻辑
│   │   ├── encoder/          # 编码逻辑
│   │   └── time_utils/       # 时间工具逻辑
│   └── utils/                # 通用工具函数
│       ├── mod.rs
│       └── input.rs          # 输入处理（文件/管道/交互）
└── tests/                    # 集成测试
```

### 分层设计

- `cli/` 和 `tui/` 是表现层，负责用户交互
- `core/` 是业务逻辑层，与 UI 无关，可复用
- `utils/` 提供通用功能支持

## 命令设计

### CLI 命令

```bash
# 数据格式化
dog format <file>           # 格式化文件
dog format -t json          # 指定输出格式（json/yaml/toml）
dog format -i               # 就地修改文件
echo '{"a":1}' | dog format # 管道输入

# 编码转换
dog encode base64 <file>    # Base64 编码
dog decode base64 <file>    # Base64 解码
dog encode url <text>       # URL 编码
dog decode url <text>       # URL 解码
dog hash md5 <file>         # 计算 MD5 哈希值
dog hash sha256 <file>      # 计算 SHA256 哈希值

# 时间工具
dog time now                # 显示当前时间戳
dog time 1609459200         # 转换时间戳为可读格式
dog time "2024-01-01"       # 转换日期为时间戳
dog time -z "Asia/Shanghai" # 指定时区

# 启动 TUI
dog tui                     # 启动交互式界面
```

### 全局选项

```bash
-o, --output <file>   # 输出到文件
-f, --format <fmt>    # 输出格式（json/yaml/toml）
-h, --help            # 显示帮助
-V, --version         # 显示版本
```

## TUI 界面设计

### 主界面布局

```
┌─────────────────────────────────────────────────────────────┐
│  CodeDog - 程序员工具箱                              [q:退出] │
├─────────────────────────────────────────────────────────────┤
│ ┌──────────────┐ ┌─────────────────────────────────────────┐│
│ │ 功能菜单     │ │ 输入区域                                ││
│ │ ──────────── │ │ ┌─────────────────────────────────────┐ ││
│ │ > 格式化     │ │ │ {"name": "code-dog", "version": 1}  │ ││
│ │   编码转换   │ │ │                                     │ ││
│ │   时间工具   │ │ │                                     │ ││
│ │   哈希计算   │ │ └─────────────────────────────────────┘ ││
│ └──────────────┘ │ 操作: [e]编辑 [p]粘贴 [c]清空 [f]文件   ││
│                   └─────────────────────────────────────────┘│
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ 输出区域                                                 │ │
│ │ ┌─────────────────────────────────────────────────────┐ │ │
│ │ │ {                                                   │ │ │
│ │ │   "name": "code-dog",                              │ │ │
│ │ │   "version": 1                                     │ │ │
│ │ │ }                                                   │ │ │
│ │ └─────────────────────────────────────────────────────┘ │ │
│ │ 操作: [y]复制 [s]保存 [w]切换格式                      │ │
│ └─────────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│ [1-4]切换模块 [Tab]切换面板 [Enter]执行 [?]帮助             │
└─────────────────────────────────────────────────────────────┘
```

### 快捷键设计

| 快捷键 | 功能 |
|--------|------|
| `1/2/3/4` | 快速切换功能模块 |
| `Tab` | 在输入/输出面板间切换 |
| `Enter` | 执行当前操作 |
| `e` | 编辑输入内容 |
| `p` | 粘贴剪贴板内容 |
| `f` | 从文件加载 |
| `y` | 复制输出到剪贴板 |
| `s` | 保存输出到文件 |
| `q` | 退出 |

## 功能模块详细设计

### 1. 数据格式化模块

**支持的格式:**
- JSON
- YAML
- TOML

**功能:**
- 格式化（美化输出）
- 压缩（最小化输出）
- 格式转换（JSON ↔ YAML ↔ TOML）
- 验证（语法检查）

### 2. 编码转换模块

**支持的编码:**
- Base64 编码/解码
- URL 编码/解码
- Unicode 编码/解码

**支持的哈希:**
- MD5
- SHA-1
- SHA-256
- SHA-512

### 3. 时间工具模块

**功能:**
- 当前时间戳显示
- 时间戳 → 可读日期
- 可读日期 → 时间戳
- 时区转换
- 日期格式化

## 测试策略

### 单元测试

- 每个 core 模块独立测试
- 使用 `#[cfg(test)]` 内联测试
- 边界条件覆盖

### 集成测试

- CLI 命令测试（使用 assert_cmd）
- TUI 交互测试
- 管道输入测试

### 覆盖率目标

- 核心逻辑: 80%+
- CLI 解析: 70%+
- TUI 组件: 60%+

## Git 管理规范

### 分支策略

| 分支 | 用途 |
|------|------|
| `main` | 稳定发布版本 |
| `develop` | 开发分支 |
| `feature/*` | 功能分支 |
| `fix/*` | 修复分支 |

### 提交规范

采用 [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: 添加 JSON 格式化功能
fix: 修复时间戳转换时区问题
docs: 更新 README 文档
test: 添加编码转换单元测试
refactor: 重构输入处理逻辑
```

## 依赖清单

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
ratatui = "0.26"
crossterm = "0.27"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
toml = "0.8"
base64 = "0.22"
url = "2"
md-5 = "0.10"
sha2 = "0.10"
chrono = "0.4"
chrono-tz = "0.8"
anyhow = "1"
thiserror = "1"
arboard = "3"

[dev-dependencies]
tempfile = "3"
assert_cmd = "2"
predicates = "3"
```

## 开发里程碑

### Phase 1: 基础框架
- [ ] 项目初始化
- [ ] CLI 命令解析
- [ ] 输入处理（文件/管道）

### Phase 2: 核心功能
- [ ] 数据格式化模块
- [ ] 编码转换模块
- [ ] 时间工具模块

### Phase 3: TUI 界面
- [ ] TUI 框架搭建
- [ ] 交互逻辑实现
- [ ] 快捷键系统

### Phase 4: 测试与文档
- [ ] 单元测试
- [ ] 集成测试
- [ ] 用户文档
