// Rust 引用和自动解引用详细分析

fn main() {
    println!("=== 引用基础演示 ===\n");

    // === 1. 原始情况 ===
    let message = String::from("Hello");
    let message_2 = &message;

    println!("直接打印 String:");
    println!("message: {}", message);      // "Hello"
    println!("message_2: {}", message_2);  // "Hello" - 引用自动解引用！

    demonstrate_reference_types();
    demonstrate_deref_coercion();
    demonstrate_memory_layout();
    demonstrate_display_trait();
}

fn demonstrate_reference_types() {
    println!("\n=== 引用类型分析 ===");

    let original = String::from("World");
    let reference = &original;

    // 类型信息
    println!("original 的类型: String");
    println!("reference 的类型: &String");

    // 内存信息
    println!("original 的地址: {:p}", &original);
    println!("reference 的值（地址）: {:p}", reference);
    println!("reference 自身的地址: {:p}", &reference);

    println!("引用指向的值: {}", *reference);  // 显式解引用
    println!("引用指向的值: {}", reference);    // 隐式解引用
}

fn demonstrate_deref_coercion() {
    println!("\n=== 解引用强制转换演示 ===");

    let s = String::from("Test");
    let s_ref = &s;

    // 显式解引用
    println!("显式解引用: {}", *s_ref);

    // 隐式解引用 (Deref coercion)
    println!("隐式解引用: {}", s_ref);

    // 在 String 方法调用中的解引用
    println!("字符串长度: {}", s_ref.len());  // 自动解引用调用 len()

    // 演示多层引用
    let double_ref = &s_ref;  // &&String
    println!("双重引用: {}", **double_ref);  // 需要双重解引用
}

fn demonstrate_memory_layout() {
    println!("\n=== 内存布局演示 ===");

    let original = String::from("Memory");
    let reference = &original;

    println!("内存布局:");
    println!("┌─────────────────────┐");
    println!("│ original (String)   │");
    println!("│ ┌───────────────┐   │");
    println!("│ │ 栈上数据       │   │");
    println!("│ │ 指针: {:p}   │", &original);
    println!("│ │ 容量: {}       │   │", original.capacity());
    println!("│ │ 长度: {}       │   │", original.len());
    println!("│ └───────────────┘   │");
    println!("│       ↓              │");
    println!("│ ┌───────────────┐   │");
    println!("│ │ 堆数据        │   │");
    println!("│ │ 'Memory'     │   │");
    println!("│ └───────────────┘   │");
    println!("└─────────────────────┘");
    println!("         ↓");
    println!("┌─────────────────────┐");
    println!("│ reference (&String)  │");
    println!("│ ┌───────────────┐   │");
    println!("│ │ 指向 original │   │");
    println!("│ │ 地址: {:p} │   │", reference);
    println!("└─────────────────────┘");

    println!("\n验证:");
    println!("reference 指向的地址: {:p}", reference);
    println!("original 的地址:     {:p}", &original);
    println!("地址相同? {}", (reference as *const String) == (&original as *const String));
}

fn demonstrate_display_trait() {
    println!("\n=== Display Trait 实现演示 ===");

    let text = String::from("Display");
    let text_ref = &text;

    // String 实现了 Display trait
    println!("String 实现 Display: {}", text);

    // &String 也"实现"了 Display (通过解引用)
    println!("&String 也'实现': {}", text_ref);

    // 这是因为 &String 实现了 Display，通过解引用到 String
    // 这就是为什么两者输出相同的原因！

    // 对比：如果我们手动实现类似行为
    custom_display_example(&text);
}

fn custom_display_example(s: &String) {
    // 这个函数接受 &String，但我们能访问 String 的内容
    // 因为在使用时会自动解引用
    println!("自定义函数中的解引用: {}", s);
    println!("显式解引用: {}", *s);
}

// === 演示不同类型引用的行为 ===
fn demonstrate_various_references() {
    println!("\n=== 不同类型引用演示 ===");

    // 整数引用
    let num = 42;
    let num_ref = &num;
    println!("整数: {}", num);      // 42
    println!("整数引用: {}", num_ref);  // 42

    // 数组引用
    let arr = [1, 2, 3];
    let arr_ref = &arr;
    println!("数组: {:?}", arr);      // [1, 2, 3]
    println!("数组引用: {:?}", arr_ref);  // [1, 2, 3]

    // 切片引用
    let slice = &arr[0..2];
    println!("切片: {:?}", slice);    // [1, 2]
}