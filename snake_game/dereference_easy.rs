// Rust 解引用（Dereference）简单易懂的演示

fn main() {
    println!("🎯 Rust 解引用简单演示\n");

    // 原始代码分析
    println!("=== 原始代码分析 ===");
    let a = 10;
    let b = &a;
    let c = &b;

    println!("a: {}", a);
    println!("b: {}", b);        // 自动解引用
    println!("c: {}", c);        // 自动解引用两次
    println!("a == *b: {}", a == *b);   // 显式解引用
    println!("a == **c: {}", a == **c);  // 双重解引用

    // 更容易理解的解引用示例
    println!("\n=== 容易理解的解引用示例 ===\n");

    // 示例1: 基础解引用
    basic_dereference_example();

    // 示例2: 多层解引用
    multi_level_example();

    // 示例3: 自动解引用
    automatic_dereference_example();

    // 示例4: 可变解引用
    mutable_dereference_example();

    // 示例5: 实用解引用场景
    practical_examples();
}

fn basic_dereference_example() {
    println!("1️⃣ 基础解引用:");

    let value = 42;
    let reference = &value;

    println!("   原始值: {}", value);
    println!("   引用: {}", reference);       // 自动解引用
    println!("   显式解引用: {}", *reference); // *操作符

    println!("   内存关系:");
    println!("   value 地址: {:p}", &value);
    println!("   reference 存储的地址: {:p}", reference);
}

fn multi_level_example() {
    println!("2️⃣ 多层解引用:");

    let original = String::from("Hello");
    let level1 = &original;    // &String
    let level2 = &level1;      // &&String
    let level3 = &level2;      // &&&String

    println!("   原始: {}", original);
    println!("   1层: {}", level1);   // 自动解引用1次
    println!("   2层: {}", level2);   // 自动解引用2次
    println!("   3层: {}", level3);   // 自动解引用3次

    println!("   显式解引用:");
    println!("   *level1: {}", *level1);   // 解引用1次
    println!("   **level2: {}", **level2);  // 解引用2次
    println!("   ***level3: {}", ***level3); // 解引用3次
}

fn automatic_dereference_example() {
    println!("3️⃣ 自动解引用场景:");

    let text = String::from("Automatic");
    let text_ref = &text;

    // 打印时自动解引用
    println!("   打印: {}", text_ref);

    // 方法调用时自动解引用
    println!("   长度: {}", text_ref.len());

    // 比较时自动解引用
    println!("   比较: {}", text_ref == "Automatic");

    // 字段访问时自动解引用
    let point = Point { x: 10, y: 20 };
    let point_ref = &point;
    println!("   字段访问: ({}, {})", point_ref.x, point_ref.y);
}

struct Point {
    x: i32,
    y: i32,
}

fn mutable_dereference_example() {
    println!("4️⃣ 可变解引用:");

    let mut value = 100;
    println!("   修改前: {}", value);

    {
        let mut_ref = &mut value;
        *mut_ref *= 2;  // 通过可变引用修改
        println!("   通过引用修改: {}", *mut_ref);
    }

    println!("   修改后: {}", value);
}

fn practical_examples() {
    println!("5️⃣ 实用解引用场景:");

    // 向量和迭代器
    let numbers = vec![1, 2, 3, 4, 5];
    let first = &numbers[0];
    println!("   向量元素: {}", *first);

    let sum: i32 = numbers.iter().map(|x| x * 2).sum();
    println!("   迭代器解引用求和: {}", sum);

    // Option 解引用
    let optional = Some(42);
    match optional {
        Some(ref value) => println!("   Option 解引用: {}", *value),
        None => println!("   无值"),
    }

    // 字符串切片
    let s = String::from("Hello World");
    let first_word = &s[0..5];
    println!("   字符串切片: {}", first_word);

    // 数组引用
    let arr = [10, 20, 30];
    let arr_ref = &arr;
    println!("   数组引用: {:?}", arr_ref);
    println!("   访问元素: {}", (*arr_ref)[0]);
}
