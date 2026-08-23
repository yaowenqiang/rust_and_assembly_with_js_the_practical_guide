// Rust 解引用（Dereference）完全演示 - 简化版

fn main() {
    println!("🎯 Rust 解引用完全演示\n");

    analyze_original_code();
    demonstrate_basic_dereference();
    demonstrate_multi_level_dereference();
    demonstrate_practical_examples();
}

fn analyze_original_code() {
    println!("=== 原始代码分析 ===\n");

    // 第1部分：字符串引用
    let message = String::from("Hello");
    let message_2 = &message;       // 不可变引用
    let message_3 = &message_2;     // 引用的引用

    println!("字符串引用链:");
    println!("message: {}", message);
    println!("message_2: {}", message_2);    // 自动解引用
    println!("message_3: {}", message_3);   // 自动解引用两次

    // 第2部分：整数引用和比较
    let a = 10;
    let b = &a;
    let c = &b;

    println!("\n整数引用和比较:");
    println!("a == *b: {}", a == *b);      // 显式解引用
    println!("a == **c: {}", a == **c);   // 双重解引用

    // 第3部分：多层引用
    let e = &&100;

    println!("\n多层引用解引用:");
    println!("**e: {}", **e);                // 解引用到值
}

fn demonstrate_basic_dereference() {
    println!("\n=== 基础解引用演示 ===\n");

    let value = 42;
    let reference = &value;

    println!("原始值: {}", value);
    println!("引用: {}", reference);         // 自动解引用
    println!("显式解引用: {}", *reference);   // 显式解引用

    println!("\n内存布局:");
    println!("value 的地址: {:p}", &value);
    println!("reference 存储的地址: {:p}", reference);
    println!("reference 自身的地址: {:p}", &reference);

    // 解引用的可变性
    let mut mutable_value = 100;
    let mutable_ref = &mut mutable_value;
    *mutable_ref += 50;                       // 通过可变引用修改值
    println!("修改后的值: {}", mutable_value);
}

fn demonstrate_multi_level_dereference() {
    println!("\n=== 多层解引用演示 ===\n");

    let original = String::from("MultiLevel");
    let level1 = &original;           // &String
    let level2 = &level1;             // &&String
    let level3 = &level2;             // &&&String

    println!("原始值: {}", original);
    println!("1层引用: {}", level1);   // 自动解引用
    println!("2层引用: {}", level2);   // 自动解引用两次
    println!("3层引用: {}", level3);   // 自动解引用三次

    // 显式多层解引用
    println!("\n显式解引用:");
    println!("*level1: {}", *level1);
    println!("**level2: {}", **level2);
    println!("***level3: {}", ***level3);

    // 类型信息
    println!("\n类型信息:");
    println!("original: String");
    println!("level1: &String");
    println!("level2: &&String");
    println!("level3: &&&String");
}

fn demonstrate_practical_examples() {
    println!("\n=== 实用解引用示例 ===\n");

    // 示例1: 基础比较
    let x = 42;
    let x_ref = &x;
    println!("比较示例:");
    println!("  x == *x_ref: {}", x == *x_ref);
    println!("  x == x_ref: {}", x == *x_ref); // 自动解引用比较

    // 示例2: 向量元素访问
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_ref = &numbers;
    println!("\n向量引用:");
    println!("  向量: {:?}", numbers);
    println!("  通过引用: {:?}", numbers_ref);
    println!("  访问元素: {}", numbers_ref[0]); // 自动解引用

    // 示例3: 字符串长度
    let text = String::from("Hello");
    let text_ref = &text;
    println!("\n字符串引用:");
    println!("  字符串: {}", text);
    println!("  长度: {}", text_ref.len()); // 自动解引用调用方法

    // 示例4: 结构体字段访问
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    let point_ref = &point;
    println!("\n结构体引用:");
    println!("  点坐标: ({}, {})", point_ref.x, point_ref.y); // 自动解引用

    // 示例5: 迭代器解引用
    let vec = vec![1, 2, 3];
    let iter = vec.iter();
    let sum: i32 = iter.map(|x| x * 2).sum();
    println!("\n迭代器解引用:");
    println!("  原向量: {:?}", vec);
    println!("  处理后求和: {}", sum);

    // 示例6: Option 解引用
    let optional = Some(42);
    let optional_ref = &optional;
    println!("\nOption 解引用:");
    println!("  Option: {:?}", optional);
    println!("  通过引用: {:?}", optional_ref);
    if let Some(val) = optional_ref {
        println!("  解引用值: {}", *val);
    }
}
