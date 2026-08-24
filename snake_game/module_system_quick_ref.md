# ⚡ Rust 模块系统快速参考

## 🔑 关键字速查

| 关键字 | 用途 | 示例 |
|--------|------|------|
| **mod** | 定义模块 | `mod animals { }` |
| **use** | 导入路径 | `use std::fmt;` |
| **pub** | 公开访问 | `pub fn fn() { }` |
| **crate** | 包根 | `crate::module::fn()` |
| **super** | 父模块 | `super::fn()` |
| **self** | 当前模块 | `self::fn()` |

## 📝 快速语法

### **定义模块**
```rust
mod name { pub fn fn() { } }
```

### **可见性**
```rust
pub fn public() {}
fn private() {}
```

### **Use 语句**
```rust
use std::collections::HashMap;
use animals::{dog, cat};
```

## 🔑 记忆口诀

### **路径规则：**
> **"crate:: 从根，super:: 向上"**

### **可见性：**
> **"默认私有，需要 pub"**

---

**详细指南**: `module_system_guide.md`  
**单文件示例**: `./module_system_examples`  
**多文件演示**: `cargo run --bin module_demo_runner`
