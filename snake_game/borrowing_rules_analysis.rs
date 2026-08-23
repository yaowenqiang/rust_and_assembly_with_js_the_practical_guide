// Rust 可变借用规则深度分析

fn main() {
    println!("=== 可变借用规则演示 ===\n");

    demonstrate_original_error();
    demonstrate_fix_approach();
    demonstrate_borrowing_rules();
    demonstrate_scopes();
}

fn demonstrate_original_error() {
    println!("=== 1. 原始错误演示 ===");
    println!("以下代码会产生编译错误：\n");

    println!("let mut message = String::from(\"Hello\");");
    println!("let message_2: &mut String = &mut message;");
    println!("message_2.push_str(\" world\");");
    println!("println!(\"{{}}\", message);      // ❌ 错误行");
    println!("println!(\"{{}}\", message_2);");

    println!("\n错误信息:");
    println!("error[E0502]: cannot borrow `message` as immutable");
    println!("because it is also borrowed as mutable");

    println!("\n错误原因分析:");
    println!("1. 第3行: 创建了可变借用 message_2");
    println!("2. 第5行: 尝试不可变借用 message");
    println!("3. 第7行: 还要使用可变借用 message_2");
    println!("4. 违反规则: 可变借用期间不能有其他借用");
}

fn demonstrate_fix_approach() {
    println!("\n=== 2. 修复方法演示 ===");

    // 方法1: 只使用可变引用
    let mut message = String::from("Hello");
    let message_2: &mut String = &mut message;
    message_2.push_str(" world");
    // println!("{}", message);  // 不使用原始变量
    println!("只使用可变引用: {}", message_2); // ✅ 正确

    // 方法2: 分离作用域
    let mut message = String::from("Hello");
    {
        let message_2: &mut String = &mut message;
        message_2.push_str(" world");
        println!("作用域内: {}", message_2);
    } // 可变借用在此结束
    println!("作用域外: {}", message); // ✅ 现在可以使用 message

    // 方法3: 使用完后才打印
    let mut message = String::from("Hello");
    let message_2: &mut String = &mut message;
    message_2.push_str(" world");
    println!("{}", message_2); // 先使用可变引用
    println!("{}", message); // 可变引用使用完后，再使用原变量 ✅
}

fn demonstrate_borrowing_rules() {
    println!("\n=== 3. Rust 借用规则详解 ===");

    println!("Rust 的三大借用规则:");
    println!("1. 任何时间，你可以有:");
    println!("   - 一个可变引用");
    println!("   - OR 多个不可变引用");
    println!("   - BUT 不能同时有可变和不可变引用");

    println!("\n规则应用:");
    println!("┌─────────────────────┐");
    println!("│ ✅ 允许的情况       │");
    println!("├─────────────────────┤");
    println!("│ &T, &T, &T          │  多个不可变引用");
    println!("│ &mut T              │  一个可变引用");
    println!("└─────────────────────┘");

    println!("┌─────────────────────┐");
    println!("│ ❌ 不允许的情况     │");
    println!("├─────────────────────┤");
    println!("│ &T, &mut T          │  不可变 + 可变");
    println!("│ &mut T, &mut T      │  多个可变引用");
    println!("└─────────────────────┘");

    demonstrate_valid_patterns();
}

fn demonstrate_valid_patterns() {
    println!("\n有效的借用模式演示:");

    // 模式1: 多个不可变引用
    let data = String::from("Shared");
    let ref1 = &data;
    let ref2 = &data;
    let ref3 = &data;
    println!("多个不可变引用: {}, {}, {}", ref1, ref2, ref3);

    // 模式2: 单个可变引用
    let mut data = String::from("Mutable");
    let mut_ref = &mut data;
    mut_ref.push_str(" data");
    println!("单个可变引用: {}", mut_ref);

    // 模式3: 分离作用域的可变引用
    let mut data = String::from("Scoped");
    {
        let mut_ref = &mut data;
        mut_ref.push_str(" mutation");
        println!("作用域内: {}", mut_ref);
    }
    let immutable_ref = &data; // 可变借用已结束
    println!("作用域外: {}", immutable_ref);
}

fn demonstrate_scopes() {
    println!("\n=== 4. 借用作用域演示 ===");

    let mut valuable = String::from("Value");
    println!("原始值: {}", valuable);

    // 可变借用开始
    let mut borrower: &mut String = &mut valuable;
    println!("可变借用创建");

    // 在可变借用期间
    borrower.push_str(" modified");
    println!("通过可变引用修改: {}", borrower);

    // 可变借用结束（borrower 不再使用）
    println!("可变借用将结束");

    // 现在可以重新借用
    let new_borrower = &valuable;
    println!("新的不可变借用: {}", new_borrower);

    // 也可以直接使用原变量
    println!("使用原变量: {}", valuable);
}

fn demonstrate_compilation_check() {
    println!("\n=== 5. 编译器检查机制 ===");

    println!("Rust 编译器如何检查借用规则:");
    println!("1. 生命周期分析");
    println!("2. 借用检查器 (Borrow Checker)");
    println!("3. 编译时确定，无运行时开销");

    println!("\n借用检查器的检查流程:");
    println!("┌─────────────────────┐");
    println!("│ 分析每个引用的生命周期 │");
    println!("│ 检查引用重叠情况     │");
    println!("│ 验证借用规则         │");
    println!("│ 拒绝违规代码         │");
    println!("└─────────────────────┘");

    println!("你的代码被拒绝的原因:");
    println!("借用检查器发现:");
    println!("  - message_2 的生命周期: 第3-7行");
    println!("  - message 的使用: 第5行");
    println!("  - 生命周期重叠! ❌");
}
