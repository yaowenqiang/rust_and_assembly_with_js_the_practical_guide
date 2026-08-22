#!/bin/bash
# Rust LLDB 调试成功指南

cat << 'EOF'
🎯 Rust LLDB 调试成功指南

❌ 遇到的问题：
- b main::print_float_numbers ❌ "no locations (pending)"
- b main::main ❌ "no locations (pending)"

✅ 解决方法：
- 使用文件名和行号而不是函数名
- Rust 函数名在编译时会被修饰（mangle）

🔧 正确的断点设置方法：

1. 按文件和行号设置断点 (最可靠)：
   (lldb) b main.rs:2              # main 函数第2行
   (lldb) b main.rs:13             # print_welcome 函数
   (lldb) b main.rs:18             # print_integer_numbers 函数
   (lldb) b main.rs:26             # print_float_numbers 函数

2. 使用完整断点命令：
   (lldb) breakpoint set --file main.rs --line 2

3. 使用正则表达式（适用于已知函数名）：
   (lldb) breakpoint set --regex print_welcome

🎮 调试会话示例：

   rust-lldb main_debug_v2
   (lldb) b main.rs:2              # 设置断点
   (lldb) r                        # 运行程序
   (lldb) frame variable           # 查看当前变量
   (lldb) x/10x &message          # 查看内存
   (lldb) disas -c 5              # 查看汇编指令
   (lldb) s                        # 单步执行
   (lldb) c                        # 继续执行

📁 已创建的调试工具：
- lldb_debug_guide.sh - 完整调试指南
- debug_demo.sh - 调试演示脚本
- main_debug_v2 - 包含完整调试信息的程序

🚀 立即开始调试：
   rust-lldb main_debug_v2
   (lldb) b main.rs:2
   (lldb) r

EOF

# 提供一个快速测试
echo "快速测试 LLDB 调试："
echo "rust-lldb main_debug_v2 << 'LLDB_CMDS'"
echo "b main.rs:2"
echo "r"
echo "frame variable"
echo "c"
echo "LLDB_CMDS"