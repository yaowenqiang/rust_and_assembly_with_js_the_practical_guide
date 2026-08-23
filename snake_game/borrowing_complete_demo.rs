// Rust 可变借用错误完整演示和修复

fn main() {
    println!("🎯 Rust 可变借用规则完整演示\n");

    demonstrate_original_error();
    demonstrate_all_fixes();
    demonstrate_borrowing_rules();
    demonstrate_practical_examples();
    demonstrate_compilation_benefits();

    println!("\n🎉 总结: Rust 借用规则确保了编译时的内存安全！");
    println!("记住: 多个读可以共存，写必须独占，读写不能混合！");
}

fn demonstrate_original_error() {
    println!("=== 原始错误演示 ===");
    println!("以下代码会产生编译错误:\n");

    println!("let mut message = String::from(\"Hello\");");
    println!("let message_2: &mut String = &mut message;");
    println!("message_2.push_str(\" world\");");
    println!("println!(\"{{}}\", message);      // ❌ 编译错误");
    println!("println!(\"{{}}\", message_2);\n");

    println!("错误信息:");
    println!("error[E0502]: cannot borrow `message` as immutable");
    println!("because it is also borrowed as mutable\n");

    println!("原因: 可变借用(message_2)和不可变借用(message)冲突！");
}

fn demonstrate_all_fixes() {
    println!("=== 四种修复方法演示 ===\n");

    // 修复1: 只使用可变引用
    let mut message = String::from("Hello");
    let message_2: &mut String = &mut message;
    message_2.push_str(" world");
    println!("方法1 - 只用可变引用: {}", message_2);
    println!();

    // 修复2: 分离作用域
    let mut message = String::from("Hello");
    {
        let message_2: &mut String = &mut message;
        message_2.push_str(" world");
        println!("方法2 - 作用域内: {}", message_2);
    }
    println!("方法2 - 作用域外: {}", message);
    println!();

    // 修复3: 按顺序使用
    let mut message = String::from("Hello");
    let message_2: &mut String = &mut message;
    message_2.push_str(" world");
    println!("方法3 - 先用可变: {}", message_2);
    println!("方法3 - 后用原值: {}", message);
    println!();

    // 修复4: 所有权转移
    let message = String::from("Hello");
    let message_2 = message;
    let mut message_4 = message_2;
    message_4.push_str(" world");
    println!("方法4 - 所有权转移: {}", message_4);
}

fn demonstrate_borrowing_rules() {
    println!("=== Rust 借用规则详解 ===\n");

    println!("三大借用规则:");
    println!("1. 任何时间，你可以有:");
    println!("   ✅ 一个可变引用 (&mut T)");
    println!("   ✅ 或多个不可变引用 (&T, &T, &T...)");
    println!("   ❌ 但不能同时有可变和不可变引用");

    demonstrate_valid_patterns();
    demonstrate_invalid_patterns();
}

fn demonstrate_valid_patterns() {
    println!("\n✅ 有效的借用模式:");

    // 多个不可变引用
    let data = String::from("Shared");
    let ref1 = &data;
    let ref2 = &data;
    let ref3 = &data;
    println!("多个不可变引用: {}, {}, {}", ref1, ref2, ref3);

    // 单个可变引用
    let mut data = String::from("Mutable");
    let mut_ref = &mut data;
    mut_ref.push_str(" modified");
    println!("单个可变引用: {}", mut_ref);

    // 分离作用域
    let mut data = String::from("Scoped");
    {
        let mut_ref = &mut data;
        mut_ref.push_str(" mutation");
        println!("作用域内可变: {}", mut_ref);
    }
    let immut_ref = &data;
    println!("作用域外不可变: {}", immut_ref);
}

fn demonstrate_invalid_patterns() {
    println!("\n❌ 无效的借用模式:");

    println!("// 模式1: 不可变 + 可变");
    println!("let mut data = String::from(\"Error\");");
    println!("let ref1 = &data;");
    println!("let ref2 = &mut data;  // ❌ 编译错误！");

    println!("\n// 模式2: 多个可变引用");
    println!("let mut data = String::from(\"Error\");");
    println!("let ref1 = &mut data;");
    println!("let ref2 = &mut data;  // ❌ 编译错误！");

    println!("\n// 模式3: 在可变借用期间使用原变量");
    println!("let mut data = String::from(\"Error\");");
    println!("let ref = &mut data;");
    println!("println!(\"{{}}\", data);  // ❌ 编译错误！");
}

fn demonstrate_practical_examples() {
    println!("\n=== 实际应用示例 ===");

    // 向量修改示例
    let mut numbers = vec![1, 2, 3, 4, 5];
    modify_vector(&mut numbers);
    println!("向量修改: {:?}", numbers);

    // 字符串处理示例
    let mut text = String::from("Hello");
    process_string(&mut text);
    println!("字符串处理: {}", text);

    // 安全的读写模式
    safe_read_write_pattern();
}

fn modify_vector(vec: &mut Vec<i32>) {
    for item in vec.iter_mut() {
        *item *= 2;
    }
}

fn process_string(s: &mut String) {
    s.push_str(" World");
    s.make_ascii_uppercase();
}

fn safe_read_write_pattern() {
    println!("安全读写模式:");

    let mut data = String::from("Pattern");

    // 先读取
    println!("读取: {}", data);

    // 再修改
    data.push_str(" modified");
    println!("修改后: {}", data);

    // 或者使用作用域分离
    let mut data = String::from("Scoped");

    {
        let modifier = &mut data;
        modifier.push_str(" in scope");
        println!("作用域内: {}", modifier);
    }

    println!("作用域外: {}", data);
}

fn demonstrate_compilation_benefits() {
    println!("\n=== 借用规则的好处 ===");

    println!("1. 🛡️ 内存安全 - 编译时保证，无运行时检查");
    println!("2. ⚡ 性能无损 - 零成本抽象，无额外开销");
    println!("3. 🔒 线程安全 - 防止数据竞争");
    println!("4. 🎯 代码清晰 - 明确的数据访问意图");
}
