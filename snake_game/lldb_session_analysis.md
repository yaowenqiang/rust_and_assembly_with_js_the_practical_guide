# 🔬 LLDB 调试会话深度分析

## 当前状态分析

你现在停在 `main.rs:3` 处：`println!("{}", message);`

## 内存布局发现

从你的 `x &message` 输出发现的 &str 结构：

```c
struct &str {
    data_ptr: *const u8,    // 8 字节 - 0x1000003c6a2
    length: usize,          // 8 字节 - 13
}                           // 总共 16 字节
```

## 🎯 建议的调试步骤

### 1. 验证字符串内容
```lldb
(lldb) p message
(lldb) p message.as_ptr()          # 应该显示 0x0000000100003c6a2
(lldb) p message.len()              # 应该显示 13
```

### 2. 查看实际字符串数据
```lldb
(lldb) x/13c 0x1000003c6a2
# 应该看到 'H' 'e' 'l' 'l' 'o' ' ' 'w' 'o' 'r' 'l' 'd' '!' '\0'
```

### 3. 单步执行观察变化
```lldb
(lldb) n                            # 执行 println!
(lldb) n                            # 执行 message = "Hello, Rust!"
(lldb) x/2gx &message              # 查看内存变化
(lldb) p message.len()             # 长度应该变成 12
```

### 4. 查看局部变量列表
```lldb
(lldb) v                           # 简写 frame variable
(lldb) frame variable              # 完整命令
```

## 🔧 常用 LLDB 命令修正

### ❌ 错误的命令 → ✅ 正确的 LLDB 命令

```
info locals      → frame variable     # 查看局部变量
info registers   → register read      # 查看寄存器
disassemble      → disas -c 10        # 反汇编 10 条指令
backtrace        → bt                 # 查看调用栈
```

## 💡 Rust 类型内存模板

### &str 类型（胖指针）
```lldb
(lldb) x/2gx &str_variable     # [指针，长度]
(lldb) p sizeof(str_variable)   # 16 字节
```

### String 类型
```lldb
(lldb) x/3gx &string_variable   # [指针，容量，长度]
(lldb) p sizeof(string_variable) # 24 字节
(lldb) p string_variable.capacity()
(lldb) p string_variable.as_ptr()
```

### Vec<T> 类型
```lldb
(lldb) x/3gx &vec_variable      # [指针，容量，长度]
(lldb) p vec_variable.len()
(lldb) p vec_variable.capacity()
```

## 🚀 继续调试的建议命令

```lldb
# 继续你的调试会话
(lldb) p message                  # 查看当前字符串值
(lldb) p message.as_ptr()         # 查看数据指针
(lldb) p message.len()            # 查看长度
(lldb) x/13c 0x1000003c6a2       # 查看原始字符串内容

# 单步执行到下一个函数调用
(lldb) n                          # 执行 println!
(lldb) n                          # 执行 message = "Hello, Rust!"
(lldb) n                          # 执行 println!("{}", message)
(lldb) n                          # 调用 print_welcome(message)

# 进入函数内部
(lldb) s                          # step into print_welcome
(lldb) v                          # 查看函数参数
(lldb) x/2gx &message            # 查看参数的内存布局
```