# 🎯 Zed 多文件 Rust 调试完整指南

## 📋 项目结构

现在你的项目支持调试多个 Rust 文件：

```
snake_game/
├── main.rs                    → snake_game 二进制
├── memory_demo.rs            → memory_demo 二进制  
├── memory_enhanced_main.rs    → memory_enhanced_main 二进制
├── Cargo.toml                ← 已更新，支持多个二进制
├── .zed/
│   ├── settings.json         ← Zed 设置
│   └── debug.json            ← 调试配置 (6个配置)
└── target/debug/
    ├── snake_game            (500KB)
    ├── memory_demo           (514KB)
    └── memory_enhanced_main  (541KB)
```

## 🚀 调试不同文件的方法

### 方法1: 使用特定的调试配置（推荐）

#### 调试 `memory_demo.rs`

1. **打开文件**: 在 Zed 中打开 `memory_demo.rs`
2. **设置断点**: 在想要的行设置断点（如第14行）
3. **打开调试面板**: `Cmd + Shift + D`
4. **选择配置**: "Debug memory_demo"
5. **开始调试**: 按 F5 或点击 ▶️

#### 调试 `memory_enhanced_main.rs`

1. **打开文件**: 在 Zed 中打开 `memory_enhanced_main.rs`
2. **设置断点**: 在想要的行设置断点
3. **打开调试面板**: `Cmd + Shift + D`
4. **选择配置**: "Debug memory_enhanced_main"
5. **开始调试**: 按 F5

#### 调试 `main.rs`

1. **打开文件**: 在 Zed 中打开 `main.rs`
2. **设置断点**: 在想要的行设置断点
3. **打开调试面板**: `Cmd + Shift + D`
4. **选择配置**: "Debug snake_game (main.rs)"
5. **开始调试**: 按 F5

### 方法2: 使用 "Debug Current File"

1. **打开任何 .rs 文件**
2. **设置断点**
3. **选择配置**: "Debug Current File"
4. **开始调试**

⚠️ **注意**: 这个方法会尝试编译当前文件作为二进制目标。

### 方法3: 使用直接调试配置

1. **先手动编译**:
   ```bash
   cargo build --bin memory_demo
   ```

2. **选择配置**: "Debug with LLDB (Direct - memory_demo)"
3. **开始调试**: 按 F5

## 🎯 可用的调试配置

在 Zed 调试面板中，你现在有 **6 个调试配置**：

### 1. **Debug Current File**
- 自动编译和调试当前打开的文件
- 最简单，但需要文件能独立编译

### 2. **Debug snake_game (main.rs)**
- 调试主程序 (`main.rs`)
- 使用 Cargo 构建

### 3. **Debug memory_demo** ⭐
- 调试内存演示程序
- 适合学习 Rust 内存管理

### 4. **Debug memory_enhanced_main** ⭐
- 调试增强版内存分析程序
- 详细的内存布局输出

### 5. **Debug with LLDB (Direct - main)**
- 直接调试已编译的 `snake_game`
- 不重新编译，速度更快

### 6. **Debug with LLDB (Direct - memory_demo)**
- 直接调试已编译的 `memory_demo`
- 快速调试已存在的二进制

## 💡 调试不同文件的建议

### 对于 `memory_demo.rs` - 内存学习
```rust
// 推荐的断点位置
fn main() {
    // 第 14 行: 查看基本类型大小
    println!("i32 大小: {} 字节", mem::size_of::<i32>()); // ← 断点

    let a = 10; // ← 断点，查看变量 a
    let b = 20;

    let tuple = (42, 3.14, true); // ← 断点，查看复合类型
}
```

**调试步骤**:
1. 选择 "Debug memory_demo" 配置
2. 在第14行设置断点
3. 按 F5 开始调试
4. 使用 F10 单步执行，观察变量变化

### 对于 `memory_enhanced_main.rs` - 深度内存分析
```rust
// 这个文件包含详细的内存分析输出
// 适合观察内存布局的变化

let message = "Hello, world!";  // ← 断点，查看 &str 结构
println!("内存地址: {:p}", &message); // ← 断点，查看输出

let heap_value = Box::new(1000); // ← 断点，查看堆分配
```

**调试步骤**:
1. 选择 "Debug memory_enhanced_main" 配置
2. 在不同类型的变量声明处设置断点
3. 观察内存输出和变量值

## 🔧 手动编译命令

如果需要手动编译特定的二进制：

```bash
# 编译所有二进制
cargo build

# 只编译 main.rs
cargo build --bin snake_game

# 只编译 memory_demo.rs
cargo build --bin memory_demo

# 只编译 memory_enhanced_main.rs
cargo build --bin memory_enhanced_main

# 运行特定二进制
cargo run --bin memory_demo
cargo run --bin memory_enhanced_main

# 清理并重新编译
cargo clean && cargo build --bin memory_demo
```

## 🎮 调试技巧示例

### Example 1: 调试 memory_demo.rs 中的变量
```lldb
# 在调试控制台中执行：
v custom_num           # 查看变量
p mem::size_of_val(&custom_num)  # 查看大小
x/4x &custom_num      # 查看内存布局
```

### Example 2: 调试 memory_enhanced_main.rs 中的字符串
```lldb
# 查看 &str 胖指针结构：
x/2gx &message         # 查看指针和长度
p message.as_ptr()     # 获取数据指针
p message.len()        # 获取长度
```

### Example 3: 观察内存分配
```lldb
# 在堆分配处设置断点：
# 停在: let heap_value = Box::new(1000);
v heap_value           # 查看 Box 内容
x/8x heap_value        # 查看堆内存地址
```

## ⚡ 快速调试流程

### 场景1: 想快速测试 memory_demo
```bash
# 在 Zed 中：
1. 打开 memory_demo.rs
2. 第 14 行设置断点
3. Cmd + Shift + D
4. 选择 "Debug memory_demo"
5. F5 开始调试
```

### 场景2: 想对比 main.rs 和 memory_enhanced_main.rs
```bash
# 先调试 main.rs：
1. 打开 main.rs
2. 选择 "Debug snake_game (main.rs)"
3. 调试完成

# 然后调试增强版本：
1. 打开 memory_enhanced_main.rs
2. 选择 "Debug memory_enhanced_main"
3. 对比内存输出
```

## 🔍 故障排除

### 问题1: 找不到特定二进制的调试配置
**解决**: 确保先编译该二进制：
```bash
cargo build --bin memory_demo
```

### 问题2: 调试时显示 "找不到文件"
**解决**: 确保在对应的 .rs 文件中设置断点，而不是在其他文件中。

### 问题3: 变量显示 "optimized out"
**解决**: 确保使用 debug 模式，不是 release 模式：
```bash
cargo build --bin memory_demo    # 正确
cargo build --bin memory_demo --release  # 变量会被优化
```

### 问题4: 多个二进制冲突
**解决**: 清理并重新编译：
```bash
cargo clean
cargo build --bin memory_demo
```

## 🚀 现在开始调试多个文件！

1. **打开 memory_demo.rs**
2. **在第 14 行设置断点**
3. **选择 "Debug memory_demo" 配置**
4. **按 F5 开始调试**

享受多文件调试的便利！🎉