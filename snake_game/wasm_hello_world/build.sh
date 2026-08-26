#!/bin/bash

echo "🔧 构建 Rust WebAssembly Hello World 项目"

# 检查 wasm-pack 是否安装
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack 未安装"
    echo "📦 正在安装 wasm-pack..."
    cargo install wasm-pack
fi

echo "📦 使用 wasm-pack 构建..."
wasm-pack build --target web --out-dir pkg

if [ $? -eq 0 ]; then
    echo "✅ 构建完成!"
    echo ""
    echo "📝 使用说明:"
    echo "1. 启动本地服务器: python3 -m http.server 8000"
    echo "2. 或者在 wasm_hello_world 目录中: python3 -m http.server 8000"
    echo "3. 打开浏览器访问: http://localhost:8000/wasm_hello_world/"
    echo ""
    echo "🎮 或直接用浏览器打开: wasm_hello_world/index.html"
else
    echo "❌ 构建失败"
    exit 1
fi
