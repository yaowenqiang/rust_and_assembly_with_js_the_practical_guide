#!/bin/bash
# LLDB 调试演示 - 自动化调试会话

echo "=== LLDB 自动调试演示 ==="
echo ""

# 1. 创建一个简单的调试会话脚本
cat > lldb_session.txt << 'EOF'
# 设置断点在文件和行号
breakpoint set --file main.rs --line 2

# 运行程序
run

# 查看当前栈帧信息
frame variable

# 查看当前汇编指令
disassemble --count 5

# 单步执行几条指令
stepi
stepi
stepi

# 查看寄存器
register read x0 x1

# 继续执行
continue
EOF

echo "2. 启动 LLDB 调试会话..."
echo "正在执行自动化调试命令..."
echo ""

# 使用 LLDB 脚本执行调试
rust-lldb main_debug_v2 --source-before-file=lldb_session.txt

echo ""
echo "=== 调试会话完成 ==="
echo ""
echo "现在你可以手动调试了："
echo "rust-lldb main_debug_v2"