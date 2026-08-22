#!/bin/bash
# LLDB 命令快速参考指南

cat << 'EOF'
🎯 LLDB 调试命令指南 (GDB vs LLDB)

❌ 常见的 GDB → LLDB 命令差异：

GDB                    LLDB
info locals    →  frame variable          # 查看局部变量
info args      →  frame variable -a       # 查看函数参数
info registers  →  register read           # 查看寄存器
disassemble    →  disassemble             # 反汇编
x/10x address  →  x/10x address           # 查看内存
next           →  next 或 n               # 单步执行(跳过)
step           →  step 或 s               # 单步执行(进入)
continue       →  continue 或 c           # 继续执行
print var      →  print var 或 p var      # 打印变量
backtrace      →  bt 或 thread backtrace  # 调用栈

✅ 正确的 LLDB 字符串查看方法：

# 方法1: 使用 frame variable (最简单)
(lldb) frame variable
(lldb) v message                      # 简写

# 方法2: 使用 print 命令
(lldb) p message
(lldb) p *message                     # 解引用

# 方法3: 查看原始字符串数据
(lldb) memory read 0x1000003c6a2 -c 13 -s 1
(lldb) x/13c 0x1000003c6a2

# 方法4: 查看结构体字段
(lldb) p message.as_ptr()            # 获取数据指针
(lldb) p message.len()               # 获取长度

🔬 分析 &str 内存布局：

# 查看完整的 &str 结构体 (16字节)
(lldb) p sizeof(message)             # 16 字节
(lldb) x/2x &message                  # 查看两个指针

# 手动解析内存内容
(lldb) x/1x &message                  # 数据指针
(lldb) x/1gx (&message + 8)           # 长度 (使用 gx 读取 8 字节)

# 字符串内容分析
(lldb) x/13c 0x1000003c6a2           # 读取 13 个字符

📊 Rust 类型内存分析模板：

# &str 类型 (胖指针，16字节)
(lldb) x/2gx &str_variable           # 两个64位值：[指针，长度]

# String 类型 (24字节栈上 + 堆内存)
(lldb) x/3gx &string_variable        # [指针，容量，长度]

# Vec<T> 类型 (24字节栈上 + 堆内存)
(lldb) x/3gx &vec_variable           # [指针，容量，长度]

# [T; N] 数组 (N * size_of<T>)
(lldb) x/5x &array_variable          # 5个元素的数组

EOF

echo "💡 针对你的调试会话的命令："
cat << 'EOF'

你现在在第3行 "println!("{}", message);" 处停下了。

建议的调试步骤：

1. 查看字符串内容和结构:
   (lldb) p message
   (lldb) p message.as_ptr()
   (lldb) p message.len()

2. 分析内存布局:
   (lldb) x/2gx &message
   (lldb) x/13c 0x1000003c6a2

3. 单步执行观察变化:
   (lldd) n                       # 执行 println!
   (lldb) n                       # 执行 message = "Hello, Rust!"
   (lldb) x/2gx &message         # 再次查看内存变化

4. 查看局部变量:
   (lldb) frame variable
   (lldb) v                      # 简写

EOF