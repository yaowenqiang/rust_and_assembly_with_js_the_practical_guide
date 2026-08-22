# ⚡ Rust REPL 快速参考

## 🎯 主要解决方案

### **⭐ Evcxr - 首选 Rust REPL**

```bash
# 安装
cargo install evcxr_repl

# 使用
evcxr

# 基础命令
:help          # 帮助
:quit          # 退出
:vars          # 查看变量
:type <expr>   # 查看类型
:dep <crate>   # 添加依赖
```

## 🚀 立即开始

```bash
# 1. 安装 Evcxr
cargo install evcxr_repl

# 2. 启动 REPL
evcxr

# 3. 开始编码
>> 1 + 2
3
>> let x = 42;
42
>> x * 2
84
```

## 🎮 使用示例

### 基础计算
```rust
>> 2 + 2
4
>> 5 * 10
50
```

### 定义变量和函数
```rust
>> let name = "Rust";
>> fn greet(who: &str) { println!("Hello, {}!", who); }
>> greet(name);
Hello, Rust!
```

### 使用外部 crate
```rust
>> :dep rand = "0.8"
>> use rand::Rng;
>> let random = rand::thread_rng().gen_range(1..100);
```

### 查看变量
```rust
>> let x = 10;
>> let y = 20;
>> :vars
x: i32 = 10
y: i32 = 20
```

### 查看类型
```rust
>> :type "hello"
&str
>> :type vec![1, 2, 3]
Vec<i32>
```

## 📱 替代方案

### 在线编译器（无需安装）
- [Rust Playground](https://play.rust-lang.org/) - 官方在线编译器
- [Rust Godbolt](https://rust.godbolt.org/) - 查看汇编输出

### Jupyter Notebook（数据科学）
```bash
cargo install evcxr_jupyter
evcxr_jupyter --install
jupyter notebook
```

### 脚本工具
```bash
cargo install rust-script
rust-script my_file.rs
```

## 💡 最适合的使用场景

| 场景 | 工具 | 原因 |
|------|------|------|
| **学习 Rust** | Evcxr REPL | 交互式，即时反馈 |
| **算法实验** | Evcxr REPL | 快速测试想法 |
| **数据分析** | Evcxr + Jupyter | 可视化和文档 |
| **快速测试** | Rust Playground | 无需安装，在线使用 |
| **查看汇编** | Rust Godbolt | 查看编译器输出 |

## 🆚 对比其他语言

| 语言 | REPL | 状态 |
|------|------|------|
| **Python** | `python` | ✅ 官方支持 |
| **Node.js** | `node` | ✅ 官方支持 |
| **Rust** | `evcxr` | ⚠️ 社区工具 |

## 🔧 故障排除

| 问题 | 解决方案 |
|------|----------|
| 安装失败 | `rustup update && cargo install evcxr_repl` |
| 依赖无法添加 | 检查网络连接和 crate 名称 |
| 类型推断错误 | 显式指定类型：`let x: i32 = 42;` |

## 🎯 快速决策

**想要 REPL?** → `cargo install evcxr_repl && evcxr`
**想要 Jupyter?** → `cargo install evcxr_jupyter && evcxr_jupyter --install`
**不想安装?** → 打开 https://play.rust-lang.org/

---

**详细指南**: `rust_repl_guide.md`  
**安装脚本**: `install_evxr.sh`