# 🎯 Rust 元编程完全指南：获取 Struct 名称

## 🎯 你的需求：Trait 中获取 Struct 名称

### **最简单的方法（推荐）**
```rust
trait Animal {
    fn show_type(&self) {
        println!("我是: {}", std::any::type_name::<Self>());
    }
}

struct Dog;
impl Animal for Dog {}

// 使用
let dog = Dog;
dog.show_type();  // 输出: 我是: Dog
```

## 🔑 三种主要方法

### **1. std::any::type_name - 编译时 (推荐)**
```rust
// 在 trait 中获取自身类型名称
trait Named {
    fn get_name(&self) -> &'static str {
        std::any::type_name::<Self>()  // 零开销，编译时确定
    }
}
```

### **2. std::any::type_name_of_val - 运行时**
```rust
// 从值获取类型名称
fn show_type<T>(value: &T) {
    println!("{}", std::any::type_name_of_val(value))  // 运行时获取
}
```

### **3. Any trait - 运行时类型检查**
```rust
use std::any::Any;

trait Animal {
    fn as_any(&self) -> &dyn Any;
}

// 使用
if let Some(dog) = animal.as_any().downcast_ref::<Dog>() {
    println!("这是 Dog!");
}
```

## 🚀 实用示例

### **示例1: 多种动物类型识别**
```rust
trait Animal {
    fn identify(&self);
}

struct Dog;
impl Animal for Dog {
    fn identify(&self) {
        println!("我是 {}", std::any::type_name::<Dog>());
    }
}

struct Cat;
impl Animal for Cat {
    fn identify(&self) {
        println!("我是 {}", std::any::type_name::<Cat>());
    }
}
```

### **示例2: 运行时类型分发**
```rust
use std::any::{Any, TypeId};

fn process_animal(animal: Box<dyn Animal>) {
    match animal.as_any().type_id() {
        id if id == TypeId::of::<Dog>() => println!("处理狗"),
        id if id == TypeId::of::<Cat>() => println!("处理猫"),
        _ => println!("未知类型"),
    }
}
```

### **示例3: 类型安全的转换**
```rust
fn try_feed<T: Animal>(animal: &T) -> &'static str {
    if let Some(_dog) = animal.as_any().downcast_ref::<Dog>() {
        "喂狗粮"
    } else if let Some(_cat) = animal.as_any().downcast_ref::<Cat>() {
        "喂猫粮"
    } else {
        "未知动物"
    }
}
```

## 📊 方法对比

| 方法 | 复杂度 | 性能 | 适用场景 |
|------|--------|------|----------|
| **type_name::\<Self\>** | 简单 | 最高 | trait 中获取自身类型 |
| **type_name_of_val** | 简单 | 高 | 泛型函数中获取类型 |
| **Any trait** | 中等 | 中 | 运行时类型检查 |

## 💡 Python vs Rust 对比

### **Python (简单但运行时)**
```python
class Dog: pass
print(Dog().__class__.__name__)  # "Dog"
```

### **Rust (类型安全，可编译时)**
```rust
struct Dog;
impl Dog {
    fn name() -> &'static str {
        std::any::type_name::<Dog>()  // 编译时确定
    }
}
println!("{}", Dog::name());  // "Dog"
```

## 🔑 记忆技巧

### **核心方法：**
> **"Rust 中获取 struct 名称用 `std::any::type_name::<Self>()`"**

### **选择原则：**
> **"能编译时就不要运行时，能简单就不要复杂"**

---

**完整示例**: `metaprogramming_examples.rs`  
**快速参考**: `metaprogramming_quick_ref.md`