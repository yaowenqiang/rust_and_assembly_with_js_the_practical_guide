# 🎯 Rust 模块系统完全指南

## 📋 核心概念

### **模块系统是什么？**
Rust 的模块系统用于组织代码，控制可见性，管理作用域。

## 🔑 关键概念

### **1. mod - 模块声明**
### **2. use - 导入语句**  
### **3. pub - 可见性控制**
### **4. crate - 包根路径**
### **5. super - 父模块路径**

## 🏗️ 项目结构

### **推荐结构**
```
src/
├── main.rs        # 二进制入口
├── lib.rs         # 库入口
├── models/        # 数据模型
└── utils/         # 工具函数
```

## 💡 快速示例

### **单文件模块**
```rust
mod animals {
    pub mod dog {
        pub fn bark() { }
    }
}
```

### **多文件模块**
```
src/animals/mod.rs: pub mod dog;
src/animals/dog.rs: pub fn bark() { }
```

## 🔑 记忆技巧

### **路径规则：**
> **"crate:: 从根开始，super:: 向上一级"**

### **可见性：**
> **"默认私有，显式公共"**

---

**完整示例**: `module_system_examples.rs`  
**多文件演示**: `cargo run --bin module_demo_runner`
