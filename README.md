# CodeDog (dog)

一个面向个人开发者的轻量级 CLI+TUI 程序员工具箱。

## 安装

```bash
cargo install --path .
```

## 使用方法

### CLI 模式

```bash
# 数据格式化
dog format <file>           # 格式化 JSON 文件
dog format -t yaml <file>   # 转换为 YAML
dog format -t toml <file>   # 转换为 TOML
dog format -i <file>        # 就地修改文件
echo '{"a":1}' | dog format # 管道输入

# 编码转换
dog encode base64 <text>    # Base64 编码
dog decode base64 <text>    # Base64 解码
dog encode url <text>       # URL 编码
dog decode url <text>       # URL 解码

# 哈希计算
dog hash md5 <file>         # 计算 MD5
dog hash sha256 <file>      # 计算 SHA256

# 时间工具
dog time now                # 当前时间
dog time 1609459200         # 时间戳转日期
dog time "2024-01-01"       # 日期转时间戳
```

### TUI 模式

```bash
dog tui                     # 启动交互式界面
```

## 快捷键

| 快捷键 | 功能 |
|--------|------|
| `1/2/3/4` | 切换功能模块 |
| `Tab` | 切换面板焦点 |
| `Enter` | 执行操作 |
| `e` | 编辑输入 |
| `p` | 粘贴 |
| `y` | 复制输出 |
| `q` | 退出 |

## 功能模块

- **格式化**: JSON/YAML/TOML 格式化与转换
- **编码**: Base64/URL/Unicode 编码解码
- **时间**: 时间戳转换、时区转换
- **哈希**: MD5/SHA1/SHA256/SHA512 计算

## 开发

```bash
# 构建
cargo build

# 运行测试
cargo test

# 运行
cargo run -- --help
```

## 许可证

MIT
