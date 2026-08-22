# ⚡ 可变性传递快速参考

## 🎯 你问题的直接答案

```rust
let b = 10;                    // b 是不可变的
extend_age(b);                 // ✅ 可以传递！

fn extend_age(mut a: u32) {    // a 是可变的
    a += 100;                  // 只修改副本
}
```

### **为什么可以？**
因为 `a` 是 `b` 的**独立副本**，它们有不同的内存地址！

## 📊 四种传递情况对比

| 原始变量 | 函数参数 | 传递方式 | 结果 |
|---------|---------|---------|------|
| `let b = 10` | `mut a: u32` | **Copy** | ✅ 可以 |
| `let mut c = 20` | `a: u32` | **Copy** | ✅ 可以 |
| `let d = 30` | `&mut d` | **引用** | ❌ 编译错误 |
| `let mut e = 40` | `&mut e` | **引用** | ✅ 可以 |

## 🔍 内存地址证明

```rust
let original = 42;
println!("original: {:p}", &original);  // 0x100

show_copy(original);
// 函数内: {:p} = 0x200  ← 完全不同的地址！
```

## 💡 核心原则

### **值传递 (Copy 类型)**
```rust
// 可变性独立！
let immutable = 10;
mutable_function(immutable);  // ✅

fn mutable_function(mut param: u32) {
    param += 100;  // 只修改副本
}
// immutable 仍然是 10，仍然是不可变的
```

### **引用传递 (需要真正修改原值)**
```rust
// 可变性必须匹配！
let mut mutable = 10;
mutable_ref_function(&mut mutable);  // ✅

fn mutable_ref_function(param: &mut u32) {
    *param += 100;  // 修改原值
}
// mutable 变成了 110
```

## 🎮 快速记忆

### 🅰️ Copy 类型 (u32, i32, f64, bool, etc.)
```rust
let x = 10;              // 可变或不可变都行
func(mut x_copy);        // ✅ 参数可以是可变或不可变
// x 不受影响
```

### 🅱️ Move 类型 (String, Vec<T>, etc.)
```rust
let s = String::new();   // 必须考虑可变性
func_ref(&mut s);        // ❌ s 必须是可变的
// s 可能被修改
```

## 🔧 实用规则

### ✅ 可以的情况
- 不可变 → 可变参数 (Copy)
- 可变 → 不可变参数 (Copy)
- 可变 → 可变引用

### ❌ 不可以的情况
- 不可变 → 可变引用
- 移动后的变量使用

## 🎯 记忆口诀

> **"值传递拷贝独立性，引用传递可变性约束"**

### 简化版：
- **Copy**：可变性独立 ✅
- **引用**：可变性约束 ⚠️

## 📝 代码模式

### 模式1: 只需处理值
```rust
let value = 42;
process(mut value_copy);  // ✅ 可变性独立
```

### 模式2: 需要修改原值
```rust
let mut value = 42;
process_mut_ref(&mut value);  // ✅ 必须匹配可变性
```

### 模式3: 返回新值
```rust
let value = 42;
let new_value = process(value);  // ✅ 原值保持不变
```

---

**详细解释**: `mutability_explained.md`
**演示代码**: `mutability_analysis.rs`