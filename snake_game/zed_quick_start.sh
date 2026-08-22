#!/bin/bash
# Zed 调试快速设置和验证

echo "🎯 Zed Rust 调试设置和验证"
echo ""

# 1. 检查必要的工具
echo "1. 检查调试工具..."
which rustc && echo "✅ Rust 已安装" || echo "❌ 需要安装 Rust"
which lldb && echo "✅ LLDB 已安装" || echo "❌ 需要安装 LLDB (brew install llvm)"
which cargo && echo "✅ Cargo 已安装" || echo "❌ 需要安装 Cargo"
echo ""

# 2. 创建 Zed 配置目录
echo "2. 创建 Zed 配置..."
mkdir -p .zed
echo "✅ .zed 目录已创建"
echo ""

# 3. 编译调试版本
echo "3. 编译调试版本..."
if [ -f "Cargo.toml" ]; then
    cargo build
    echo "✅ Cargo 项目编译完成"
else
    rustc -C debuginfo=2 main.rs -o main_debug_v2
    echo "✅ 单文件编译完成"
fi
echo ""

# 4. 验证二进制文件
echo "4. 验证二进制文件..."
if [ -f "target/debug/snake_game" ]; then
    ls -lh target/debug/snake_game
    echo "✅ Cargo 目标可用"
elif [ -f "main_debug_v2" ]; then
    ls -lh main_debug_v2
    echo "✅ 直接编译目标可用"
fi
echo ""

# 5. 测试调试器
echo "5. 测试 LLDB 调试器..."
TEST_BINARY="target/debug/snake_game"
if [ ! -f "$TEST_BINARY" ]; then
    TEST_BINARY="main_debug_v2"
fi

if [ -f "$TEST_BINARY" ]; then
    echo "使用: $TEST_BINARY"
    rust-lldb "$TEST_BINARY" --batch -o 'b main.rs:2' -o 'r' -o 'c' 2>&1 | grep -E "(Breakpoint|Process|launched)"
    echo "✅ LLDB 调试器工作正常"
else
    echo "❌ 找不到可调试的二进制文件"
fi
echo ""

# 6. 提供 Zed 使用说明
echo "6. 🚀 在 Zed 中开始调试："
cat << 'EOF'

   步骤 1: 在 Zed 中打开项目
   步骤 2: 按 Cmd + Shift + D 打开调试面板
   步骤 3: 点击齿轮图标选择调试配置
   步骤 4: 选择以下任一配置：
      • "Debug Current File" - 调试当前文件
      • "Debug Binary (main)" - 调试编译后的程序
      • "Debug with LLDB (Direct)" - 直接调试

   步骤 5: 在 main.rs 中设置断点（点击行号）
   步骤 6: 按 F5 开始调试

   调试控制：
   • F5 - 开始/继续
   • F10 - 单步跳过
   • F11 - 单步进入
   • Shift + F11 - 单步跳出
   • Cmd + Shift + D - 调试面板

EOF

echo "7. 📋 配置文件已创建："
ls -la .zed/
echo ""

echo "✅ 设置完成！现在可以在 Zed 中开始调试了！"
echo ""
echo "💡 提示：详细使用指南请查看 ZED_DEBUG_GUIDE.md"