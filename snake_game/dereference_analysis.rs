// Rust 解引用（Dereference）深度分析

fn main() {
    println!("🎯 Rust 解引用完全演示\n");

    analyze_original_code();
    demonstrate_basic_dereference();
    demonstrate_multi_level_dereference();
    demonstrate_automatic_dereference();
    demonstrate_practical_examples();
}

fn analyze_original_code() {
    println!("=== 原始代码分析 ===\n");

    // 第1部分：字符串引用
    let mut message = String::from("Hello");
    let message_2 = &message;       // 不可变引用
    let message_3 = &message_2;     // 引用的引用

    println!("字符串引用链:");
    println!("message: {}", message);
    println!("message_2: {}", message_2);    // 自动解引用
    println!("message_3: {}", message_3);   // 自动解引用两次

    // 第2部分：整数引用和比较
    let a = 10;
    let b = &a;
    let mut c = &b;
    let d = b;

    println!("\n整数引用和比较:");
    println!("a == *b: {}", a == *b);      // 显式解引用
    println!("a == **c: {}", a == **c);   // 双重解引用

    // 第3部分：多层引用
    let e = &&100;
    c = e;

    println!("\n多层引用解引用:");
    println!("**e: {}", **e);                // 解引用到值
    println!("*c: {:p}", *c);                // 解引用一次
    println!("**c: {:p}", **c);              // 解引用两次
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

fn demonstrate_automatic_dereference() {
    println!("\n=== 自动解引用机制 ===\n");

    let text = String::from("AutoDeref");
    let text_ref = &text;

    println!("自动解引用场景:");

    // 1. 打印时
    println!("打印: {}", text_ref);        // 自动解引用

    // 2. 方法调用时
    println!("长度: {}", text_ref.len());  // 自动解引用调用方法

    // 3. 比较时
    println!("比较: {}", text_ref == "AutoDeref"); // 自动解引用比较

    // 4. 字段访问时
    let point = Point { x: 10, y: 20 };
    let point_ref = &point;
    println!("字段访问: ({}, {})", point_ref.x, point_ref.y); // 自动解引用
}

struct Point {
    x: i32,
    y: i32,
}

fn demonstrate_practical_examples() {
    println!("\n=== 实用解引用示例 ===\n");

    // 示例1: 函数参数解引用
    demonstrate_function_dereference();

    // 示例2: 迭代器解引用
    demonstrate_iterator_dereference();

    // 示例3: 智能指针解引用
    demonstrate_smart_pointer_dereference();

    // 示例4: 解引用 coercion
    demonstrate_deref_coercion();
}

fn demonstrate_function_dereference() {
    println!("函数参数解引用:");

    let value = 42;
    let reference = &value;

    // 函数接受 i32，但我们可以传递 &i32
    print_value(reference);  // 自动解引用
    print_value(value);      // 直接传递

    fn print_value(val: i32) {
        println!("  函数接收到: {}", val);
    }
}

fn demonstrate_iterator_dereference() {
    println!("迭代器解引用:");

    let numbers = vec![1, 2, 3, 4, 5];
    let number_refs: Vec<&i32> = numbers.iter().collect();

    println!("  引用向量: {:?}", number_refs);

    // 解引用迭代
    let sum: i32 = number_refs.iter().map(|x| **x).sum();
    println!("  解引用求和: {}", sum);

    // 自动解引用
    let sum_auto: i32 = number_refs.iter().map(|x| *x).sum();
    println!("  自动解引用求和: {}", sum_auto);
}

fn demonstrate_smart_pointer_dereference() {
    println!("智能指针解引用:");

    use std::rc::Rc;

    let rc_value = Rc::new(42);
    let rc_ref = &rc_value;

    println!("  Rc 引用: {}", rc_ref);      // 自动解引用
    println!("  显式解引用: {}", **rc_ref);  // 双重解引用

    // Box 解引用
    let boxed_value = Box::new(100);
    let box_ref = &boxed_value;

    println!("  Box 引用: {}", box_ref);     // 自动解引用
    println!("  显式解引用: {}", **box_ref); // 双重解引用
}

fn demonstrate_deref_coercion() {
    println!("解引用强制转换 (Deref Coercion):");

    let string = String::from("Coercion");
    let string_ref = &string;

    // String 可以被当作 &str 使用
    fn print_str(s: &str) {
        println!("  接收到 &str: {}", s);
    }

    print_str(string_ref);  // 自动从 &String 转换为 &str
    print_str(&string);     // 同样的转换
}