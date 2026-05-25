#!/bin/bash
# 本地打包脚本

set -e

VERSION=$(grep '^version =' Cargo.toml | head -1 | sed 's/.*"\([^"]*\)".*/\1/')
DIST_DIR="dist"
BINARY_NAME="dog"

echo "=== CodeDog 打包脚本 ==="
echo "版本: $VERSION"

# 清理并创建 dist 目录
rm -rf $DIST_DIR
mkdir -p $DIST_DIR

# 构建
echo "构建发布版本..."
cargo build --release

# 检测操作系统
OS="$(uname -s)"
case "$OS" in
    Linux*)
        echo "打包 Linux 版本..."
        cp target/release/$BINARY_NAME $DIST_DIR/$BINARY_NAME-linux-x86_64
        tar -czvf $DIST_DIR/$BINARY_NAME-linux-x86_64.tar.gz -C $DIST_DIR $BINARY_NAME-linux-x86_64
        echo "已创建: $DIST_DIR/$BINARY_NAME-linux-x86_64.tar.gz"
        ;;
    Darwin*)
        echo "打包 macOS 版本..."
        cp target/release/$BINARY_NAME $DIST_DIR/$BINARY_NAME-macos-x86_64
        tar -czvf $DIST_DIR/$BINARY_NAME-macos-x86_64.tar.gz -C $DIST_DIR $BINARY_NAME-macos-x86_64
        echo "已创建: $DIST_DIR/$BINARY_NAME-macos-x86_64.tar.gz"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "打包 Windows 版本..."
        cp target/release/$BINARY_NAME.exe $DIST_DIR/$BINARY_NAME-windows-x86_64.exe
        cd $DIST_DIR
        7z a $BINARY_NAME-windows-x86_64.zip $BINARY_NAME-windows-x86_64.exe
        cd ..
        echo "已创建: $DIST_DIR/$BINARY_NAME-windows-x86_64.zip"
        ;;
    *)
        echo "未知操作系统: $OS"
        exit 1
        ;;
esac

# 生成 SHA256 校验和
echo "生成校验和..."
cd $DIST_DIR
sha256sum *.tar.gz *.zip 2>/dev/null || sha256 -r *.tar.gz *.zip 2>/dev/null || certutil -hashfile *.zip SHA256
cd ..

echo "=== 打包完成 ==="
ls -la $DIST_DIR/
