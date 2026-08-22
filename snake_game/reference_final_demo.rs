// Rust 引用和自动解引用终极演示

fn main() {
    println!("🎯 Rust 引用和自动解引用完全演示\n");

    demonstrate_basic_reference();
    demonstrate_automatic_deref();
    demonstrate_memory_addresses();
    demonstrate_display_trait();
    demonstrate_practical_examples();

    println!("\n🎉 总结：打印引用时，Rust 自动解引用到原始值！");
}

fn demonstrate_basic_reference() {
    println!("=== 1. 基础引用演示 ===");

    let message = String::from("Hello");
    let message_2 = &message;

    println!("原始值打印: {}", message);     // "Hello"
    println!("引用打印:   {}", message_2);  // "Hello" - 相同！

    println!("类型验证:");
    println!("  message 类型:   String");
    println!("  message_2 类型: &String");

    println!("地址关系:");
    println!("  message 地址:   {:p}", &message);
    println!("  message_2 值:   {:p}", message_2);  // 存储的是 message 的地址
}

fn demonstrate_automatic_deref() {
    println!("\n=== 2. 自动解引用演示 ===");

    let text = String::from("AutoDeref");
    let text_ref = &text;

    println!("显式解引用: {}", *text_ref);
    println!("隐式解引用: {}", text_ref);    // 自动解引用
    println!("完全相同? {}", *text_ref == *text_ref);  // 修复比较

    println!("方法调用中的自动解引用:");
    println!("  text_ref.len() = {}", text_ref.len());  // 自动解引用调用方法
    println!("  text.len()    = {}", text.len());
}

fn demonstrate_memory_addresses() {
    println!("\n=== 3. 内存地址关系演示 ===");

    let original = String::from("MemoryTest");
    let reference = &original;

    println!("内存布局:");
    println!("┌──────────────────┐");
    println!("│ original         │");
    println!("│ 地址: {:p}", &original);
    println!("│ 容量: {}", original.capacity());
    println!("│ 长度: {}", original.len());
    println!("└──────────────────┘");
    println!("        ↓");
    println!("┌──────────────────┐");
    println!("│ reference        │");
    println!("│ 存储: {:p}", reference);
    println!("│ 指向: original   │");
    println!("└──────────────────┘");

    println!("验证指向相同:");
    println!("  *reference == original? {}", *reference == original);
    println!("  地址相同? {}", (reference as *const String) == (&original as *const String));
}

fn demonstrate_display_trait() {
    println!("\n=== 4. Display Trait 行为演示 ===");

    let display_string = String::from("DisplayTest");
    let display_ref = &display_string;

    println!("Display trait 实现:");
    println!("  String 实现 Display: {}", display_string);
    println!("  &String 也'实现':    {}", display_ref);

    println!("这是因为 println! 宏会:");
    println!("  1. 接收 &String");
    println!("  2. 自动解引用到 String");
    println!("  3. 调用 String 的 Display 实现");
}

fn demonstrate_practical_examples() {
    println!("\n=== 5. 实用示例演示 ===");

    // 字符串切片引用
    let sentence = String::from("Hello World");
    let first_word = &sentence[0..5];
    println!("字符串切片: '{}' vs 原串: '{}'", first_word, sentence);

    // 数组引用
    let numbers = [1, 2, 3, 4, 5];
    let numbers_ref = &numbers;
    println!("数组: {:?}", numbers);
    println!("数组引用: {:?}", numbers_ref);  // 自动解引用

    // 向量引用
    let vec_data = vec![10, 20, 30];
    let vec_ref = &vec_data;
    println!("向量: {:?}", vec_data);
    println!("向量引用: {:?}", vec_ref);  // 自动解引用

    // 演示引用在函数中的使用
    demonstrate_function_references(&vec_data);
}

fn demonstrate_function_references(data: &Vec<i32>) {
    println!("函数中的引用使用:");
    println!("  接收的引用: {:?}", data);  // 自动解引用
    println!("  第一个元素: {}", data[0]);  // 自动解引用访问
    println!("  长度: {}", data.len());     // 自动解引用调用方法

    println!("这证明了引用在函数中也是自动解引用的！");
}