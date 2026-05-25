# CodeDog Windows 打包脚本
# 用法: .\scripts\package.ps1

$ErrorActionPreference = "Stop"
$DIST_DIR = "dist"
$BINARY_NAME = "dog"

# 获取版本号
$cargoContent = Get-Content "Cargo.toml" -Raw
if ($cargoContent -match 'version\s*=\s*"([^"]*)"') {
    $version = $Matches[1]
} else {
    $version = "unknown"
}

Write-Host "=== CodeDog 打包脚本 ===" -ForegroundColor Cyan
Write-Host "版本: $version"

# 清理并创建 dist 目录
if (Test-Path $DIST_DIR) {
    Remove-Item -Recurse -Force $DIST_DIR
}
New-Item -ItemType Directory -Path $DIST_DIR | Out-Null

# 构建
Write-Host "构建发布版本..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "构建失败!" -ForegroundColor Red
    exit 1
}

# 复制可执行文件
Write-Host "打包 Windows 版本..." -ForegroundColor Yellow
Copy-Item "target\release\$BINARY_NAME.exe" "$DIST_DIR\$BINARY_NAME-windows-x86_64.exe"

# 创建 ZIP 压缩包
Compress-Archive -Path "$DIST_DIR\$BINARY_NAME-windows-x86_64.exe" -DestinationPath "$DIST_DIR\$BINARY_NAME-windows-x86_64.zip" -Force

# 生成 SHA256 校验和
Write-Host "生成校验和..." -ForegroundColor Yellow
certutil -hashfile "$DIST_DIR\$BINARY_NAME-windows-x86_64.zip" SHA256 | Out-File "$DIST_DIR\SHA256SUMS.txt" -Encoding ASCII

Write-Host "=== 打包完成 ===" -ForegroundColor Green
Get-ChildItem $DIST_DIR

Write-Host ""
Write-Host "文件位置:" -ForegroundColor Cyan
Write-Host "  $PWD\$DIST_DIR\$BINARY_NAME-windows-x86_64.zip"
