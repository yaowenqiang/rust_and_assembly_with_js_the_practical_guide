# ⚡ Rust 元编程快速参考

## 🎯 你的问题解决方案

### **一行代码解决：**
```rust
trait Animal {
    fn show_type(&self) {
        println!("{}", std::any::type_name::<Self>());
    }
}
```

## 🔑 三种方法对比

| 方法 | 用途 | 代码示例 |
|------|------|----------|
| **编译时** | trait 中获取类型 | `std::any::type_name::<Self>()` |
| **运行时** | 泛型函数获取类型 | `std::any::type_name_of_val(value)` |
| **类型检查** | 运行时类型转换 | `animal.as_any().downcast_ref::<Dog>()` |

## 🚀 实用模板

### **模板1: 自动类型名称**
```rust
trait TypeName {
    fn show_name(&self) {
        println!("{}", std::any::type_name::<Self>());
    }
}
```

### **模板2: 运行时分发**
```rust
fn process<T: Animal>(animal: &T) {
    if let Some(dog) = animal.as_any().downcast_ref::<Dog>() {
        // 处理狗
    }
}
```

## 💡 核心要点

**Python**: `obj.__class__.__name__` (运行时，简单)
**Rust**: `std::any::type_name::<Self>()` (可编译时，类型安全)

> **一句话**: 用 `std::any::type_name::<Self>()` 在 trait 中获取 struct 名称！

---

**详细指南**: `metaprogramming_guide.md`  
**完整示例**: `metaprogramming_examples.rs`