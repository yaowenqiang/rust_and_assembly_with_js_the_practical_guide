# Rust 内存分析完全指南

## 🚀 快速开始

```bash
# 1. 运行内存演示程序
rustc memory_demo.rs -o memory_demo && ./memory_demo

# 2. 分析编译后的程序
./analyze_memory.sh memory_demo

# 3. 使用调试器查看运行时内存
rust-lldb memory_demo
```

## 📊 五大内存分析方法

### 1. 编译时分析 - 使用 `std::mem`

**最适合:** 查看类型大小、对齐、栈上布局

```rust
use std::mem;

fn main() {
    let x: i32 = 42;
    let s: String = String::from("hello");
    
    // 查看大小
    println!("i32 大小: {}", mem::size_of::<i32>());
    println!("String 栈上大小: {}", mem::size_of::<String>());
    
    // 查看具体变量的信息
    println!("x: 大小={}, 对齐={}", 
        mem::size_of_val(&x), 
        mem::align_of_val(&x));
    
    // 查看内存地址
    println!("x 地址: {:p}", &x);
    println!("s 地址: {:p}", &s);
    println!("s 堆数据地址: {:?}", s.as_ptr());
}
```

### 2. 汇编代码分析

**最适合:** 理解编译器如何组织内存

```bash
# 生成汇编代码
rustc --emit asm source.rs -o source.asm

# 生成 LLVM IR
rustc --emit llvm-ir source.rs -o source.ll

# 查看关键部分
grep -A 20 "main:" source.asm
```

### 3. 调试器运行时分析

**最适合:** 查看程序执行时的实际内存状态

```bash
# 启动 LLDB (macOS/Linux)
rust-lldb target/debug/program

# 常用命令
(lldb) b main                    # 设置断点
(lldb) r                         # 运行程序
(lldb) x/10x &variable           # 十六进制查看内存
(lldb) p variable                # 打印变量值  
(lldb) p sizeof(variable)        # 查看大小
(lldb) info locals               # 查看所有局部变量
(lldb) memory region start end   # 查看内存区域
(lldb) step                      # 单步执行
```

### 4. 内存分析工具

**最适合:** 检测内存泄漏、堆分配统计

```bash
# macOS 系统
dtrace -n 'syscall::malloc:return { @ = count(); }' -c "./program"

# 安装 cargo-valgrind (Linux)
cargo install cargo-valgrind
cargo valgrind run

# 使用 heaptrack (Linux)
heaptrack ./program
heaptrack_print --heap-after=program.heaptrack

# Massif (Valgrind 工具)
valgrind --tool=massif ./program
ms_print massif.out.xxxxx
```

### 5. 二进制分析

**最适合:** 分析编译后的程序布局

```bash
# 查看程序段大小
size program

# 查看文件大小
ls -lh program

# 查看符号表
nm program | grep main

# 查看动态库依赖  
otool -L program        # macOS
ldd program             # Linux

# 查看反汇编
otool -tV program       # macOS
objdump -d program      # Linux
```

## 🎯 不同场景的推荐方法

### 学习 Rust 内存管理
→ **方法 1 + 2** (std::mem + 汇编分析)
运行 `memory_demo.rs` 查看各种类型的内存布局

### 调试内存问题  
→ **方法 3** (LLDB 调试)
设置断点查看变量在运行时的内存状态

### 性能优化
→ **方法 4** (内存分析工具) + **方法 5** (二进制分析)
分析堆分配频率、程序大小、内存占用

### 理解编译器优化
→ **方法 2** (汇编对比)
```bash
rustc --emit asm --release source.rs -o release.asm
rustc --emit asm --debug source.rs -o debug.asm
diff release.asm debug.asm
```

## 📁 本项目提供的工具

- **memory_demo.rs** - 综合内存演示程序
- **analyze_memory.sh** - 一键内存分析脚本  
- **debug_memory.sh** - LLDB 调试快速启动
- **heaptrack_demo.sh** - 内存工具检查和示例

## 🔧 安装额外工具

```bash
# macOS
brew install llvm     # 包含 lldb
brew install dtrace   # 系统自带

# Linux  
sudo apt install lldb valgrind heaptrack
cargo install cargo-valgrind
```

## 💡 内存分析最佳实践

1. **从小程序开始** - 先用 `memory_demo.rs` 理解基础
2. **结合多种方法** - 编译时 + 运行时分析互补
3. **关注热点** - 重点分析频繁调用的代码
4. **对比 debug/release** - 优化的影响巨大
5. **实际测量** - 理论分析 + 实际运行数据

运行 `./analyze_memory.sh memory_demo` 查看完整的内存分析示例！