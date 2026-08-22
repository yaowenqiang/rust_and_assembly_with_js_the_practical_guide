# ⚡ Zed 多文件调试快速参考

## 🎯 3步调试任何 .rs 文件

### 调试 `memory_demo.rs`
1. **打开文件**: `memory_demo.rs`
2. **设置断点**: 第 14 行（点击行号）
3. **调试**: `Cmd + Shift + D` → 选择 "Debug memory_demo" → `F5`

### 调试 `memory_enhanced_main.rs`
1. **打开文件**: `memory_enhanced_main.rs`
2. **设置断点**: 任意想停止的行
3. **调试**: `Cmd + Shift + D` → 选择 "Debug memory_enhanced_main" → `F5`

### 调试 `main.rs`
1. **打开文件**: `main.rs`
2. **设置断点**: 第 2 行
3. **调试**: `Cmd + Shift + D` → 选择 "Debug snake_game (main.rs)" → `F5`

## 🔥 6个调试配置一览

| 配置名称 | 用途 | 文件 | 推荐度 |
|---------|------|------|--------|
| **Debug memory_demo** | 内存演示学习 | `memory_demo.rs` | ⭐⭐⭐ |
| **Debug memory_enhanced_main** | 深度内存分析 | `memory_enhanced_main.rs` | ⭐⭐⭐ |
| **Debug snake_game** | 主程序调试 | `main.rs` | ⭐⭐ |
| **Debug Current File** | 当前文件 | 任意 .rs | ⭐ |
| **Debug with LLDB (Direct - memory_demo)** | 快速调试 | `memory_demo.rs` | ⭐ |
| **Debug with LLDB (Direct - main)** | 快速调试 | `main.rs` | ⭐ |

## 💡 推荐的断点位置

### memory_demo.rs
```rust
第 14 行: println!("i32 大小: {} 字节", mem::size_of::<i32>()); // ← 查看基本类型
第 22 行: let a = 10;  // ← 查看变量创建
第 35 行: let heap_value = Box::new(1000);  // ← 查看堆分配
```

### memory_enhanced_main.rs
```rust
第 9 行:  let mut message = "Hello, world!";  // ← 查看 &str 结构
第 42 行: let custom_num = 90_000;  // ← 查看整数类型
第 69 行: let string_literal = "hello";  // ← 查看字符串对比
```

### main.rs
```rust
第 2 行: let mut message = "Hello, world!";  // ← 变量创建
第 13 行: fn print_welcome(message: &str) -> &'static str  // ← 函数调用
第 18 行: let custom_num = 90_000;  // ← 数值类型
```

## ⚡ 快捷命令

### 编译特定文件
```bash
cargo build --bin memory_demo              # 编译 memory_demo
cargo build --bin memory_enhanced_main     # 编译增强版
cargo build --bin snake_game               # 编译主程序
cargo build                                # 编译所有
```

### 运行特定文件
```bash
cargo run --bin memory_demo                # 运行 memory_demo
cargo run --bin memory_enhanced_main       # 运行增强版
cargo run --bin snake_game                 # 运行主程序
./target/debug/memory_demo                 # 直接运行
```

## 🔧 调试控制台命令示例

### 在 memory_demo 调试时
```lldb
v custom_num                  # 查看变量值
p mem::size_of_val(&custom_num)  # 查看大小
x/4x &custom_num              # 查看内存
```

### 在 memory_enhanced_main 调试时
```lldb
v message                     # 查看字符串
x/2gx &message               # 查看 &str 胖指针结构
p message.as_ptr()            # 获取数据指针
p message.len()               # 获取长度
```

## 🚨 常见问题速解

| 问题 | 解决方案 |
|------|----------|
| 找不到配置 | 先运行: `cargo build --bin memory_demo` |
| 变量 "optimized out" | 确保使用 `cargo build` 而不是 `--release` |
| 断点不工作 | 在对应文件中设置断点，不要跨文件 |
| 编译错误 | 运行: `cargo clean && cargo build` |

## 🎮 完整调试流程

```
打开 Zed → 打开 .rs 文件 → 设置断点 →
Cmd + Shift + D → 选择配置 → F5 开始调试 →
F10/F11 单步执行 → 观察变量 → 完成！
```

## 📊 文件大小参考

- `snake_game`: 500KB - 基础示例
- `memory_demo`: 514KB - 内存类型演示
- `memory_enhanced_main`: 541KB - 详细内存分析

---

**详细指南**: `ZED_MULTI_FILE_DEBUG.md`  
**快速参考**: `ZED_QUICK_REFERENCE.md`