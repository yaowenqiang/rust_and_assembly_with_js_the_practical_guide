#!/bin/bash
# Rust LLDB 调试完整指南

echo "=== Rust LLDB 调试解决方案 ==="
echo ""

# 1. 检查可用的二进制文件
echo "1. 可用的调试版本:"
ls -lh main* 2>/dev/null || echo "没有找到 main 程序"
echo ""

# 2. 重新编译调试版本
echo "2. 重新编译调试版本..."
rustc -C debuginfo=2 main.rs -o main_debug_v2
echo "✓ 调试版本编译完成: main_debug_v2"
echo ""

# 3. 检查调试信息
echo "3. 检查调试符号:"
echo "主函数符号:"
nm -a main_debug_v2 | grep " T _main"
echo ""

# 4. 提供 LLDB 调试命令
echo "=== LLDB 调试正确方法 ==="
cat << 'EOF'

# 启动 LLDB
rust-lldb main_debug_v2

# 方法1: 使用文件名和行号设置断点 (最可靠!)
(lldb) breakpoint set --file main.rs --line 2

# 方法2: 使用地址设置断点
(lldb) b 0x100001504           # 使用具体的函数地址

# 方法3: 使用正则表达式查找符号
(lldb) breakpoint set --regex print_welcome

# 方法4: 列出所有符号然后设置断点
(lldb) image lookup -n main    # 查找 main 符号
(lldb) b *0x<address>          # 使用找到的地址

# 运行和调试
(lldb) r                       # 运行程序
(lldb) s                       # 单步执行
(lldb) n                       # 单步执行(跳过函数)
(lldb) c                       # 继续执行

# 查看变量和内存
(lldb) frame variable          # 查看当前帧的变量
(lldb) print message           # 打印变量值
(lldb) x/10x &message          # 查看内存内容
(lldb) disas -c 5             # 查看当前汇编指令

# 查看调用栈
(lldb) bt                      # 查看调用栈
(lldb) up                      # 上一帧
(lldb) down                    # 下一帧

# 查看所有断点
(lldb) breakpoint list
(lldb) breakpoint disable 1    # 禁用断点
(lldb) breakpoint delete 1     # 删除断点
(lldb) breakpoint delete       # 删除所有断点

EOF

echo "=== 快速启动脚本 ==="
cat << 'EOF'

# 创建一个快速的调试启动脚本:
rust-lldb main_debug_v2 << 'LLDB_COMMANDS'
b main.rs:2
r
frame variable
disas -c 3
c
LLDB_COMMANDS

EOF

echo "5. 现在试试调试版本!"
echo "运行: rust-lldb main_debug_v2"
echo "然后使用: breakpoint set --file main.rs --line 2"