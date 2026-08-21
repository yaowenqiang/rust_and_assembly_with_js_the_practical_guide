#!/bin/bash
# Rust 内存分析工具集合

echo "=== Rust 内存分析工具 ==="
echo ""

# 1. 检查可用的内存分析工具
echo "1. 检查可用的内存分析工具："
command -v valgrind >/dev/null 2>&1 && echo "✓ valgrind 已安装" || echo "✗ valgrind 未安装"
command -v heaptrack >/dev/null 2>&1 && echo "✓ heaptrack 已安装" || echo "✗ heaptrack 未安装"
command -v massif >/dev/null 2>&1 && echo "✓ massif 已安装" || echo "✗ massif 未安装"
command -v dtrace >/dev/null 2>&1 && echo "✓ dtrace 已安装" || echo "✗ dtrace 未安装"
echo ""

# 2. 使用 rust 内置工具分析
echo "2. 使用 Rust 内置的 MIR 分析："
echo "   cargo rustc --profile=check -- --emit mir"
echo ""

# 3. 使用 GDB/LLDB 调试内存
echo "3. 使用 LLDB 调试内存示例："
cat << 'EOF'
   rust-lldb target/debug/your_program
   (lldb) b main
   (lldb) r
   (lldb) x/10x &variable_name    # 十六进制查看内存
   (lldb) p variable_name         # 打印变量值
   (lldb) info locals             # 查看所有局部变量
   (lldb) memory region           # 查看内存区域
EOF
echo ""

# 4. 实用的内存分析命令
echo "4. 实用的内存分析命令："
echo "   查看二进制文件大小: ls -lh target/release/your_program"
echo "   查看段大小: size target/release/your_program"
echo "   查看符号: nm target/release/your_program"
echo "   查看依赖: otool -L target/release/your_program"
echo ""

# 5. 创建分析脚本
if [ "$1" == "analyze" ]; then
    echo "5. 分析内存布局："
    size memory_demo 2>/dev/null || echo "   请先编译: rustc memory_demo.rs"
    ls -lh memory_demo 2>/dev/null || echo "   编译后的文件大小"
fi