#!/bin/bash
# 使用 rust-lldb 调试并查看内存
echo "启动 rust-lldb 调试会话"
echo "命令示例："
echo "  b main          # 设置断点"
echo "  r               # 运行"
echo "  x &a            # 查看变量a的内存地址"
echo "  p sizeof(a)     # 查看变量大小"
echo "  info locals     # 查看所有局部变量"
echo "  step            # 单步执行"
rust-lldb main
