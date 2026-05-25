# 更新日志

所有值得注意的变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

## [0.1.0] - 2026-05-25

### 新增
- CLI 命令行界面
  - `dog format` - 数据格式化（JSON/YAML/TOML）
  - `dog encode` - 编码转换（Base64/URL/Unicode）
  - `dog decode` - 解码（Base64/URL/Unicode）
  - `dog hash` - 哈希计算（MD5/SHA1/SHA256/SHA512）
  - `dog time` - 时间工具（时间戳转换、时区支持）
  - `dog tui` - 交互式 TUI 界面
- 支持管道输入
- 支持文件输入
- 多时区支持
- 剪贴板操作
- GitHub Actions CI/CD
  - 自动化测试（Linux/Windows/macOS）
  - 自动化构建
  - 自动发布

### 技术栈
- Rust 2021 Edition
- clap 4.x - CLI 框架
- ratatui 0.26 - TUI 框架
- serde - 数据序列化
- chrono - 时间处理
