// 修复后的可变借用代码

fn main() {
    println!("=== 可变借用修复演示 ===\n");

    // === 修复方法1: 只使用可变引用 ===
    fix_method_1();

    // === 修复方法2: 分离作用域 ===
    fix_method_2();

    // === 修复方法3: 按顺序使用 ===
    fix_method_3();

    // === 修复方法4: 避免冲突 ===
    fix_method_4();
}

fn fix_method_1() {
    println!("=== 方法1: 只使用可变引用 ===");

    let mut message = String::from("Hello");
    let message_2: &mut String = &mut message;
    message_2.push_str(" world");
    // println!("{}", message);  // 不使用原始变量，避免冲突
    println!("只使用可变引用: {}\n", message_2);  // ✅ 正确
}

fn fix_method_2() {
    println!("=== 方法2: 分离作用域 ===");

    let mut message = String::from("Hello");
    {
        let message_2: &mut String = &mut message;
        message_2.push_str(" world");
        println!("作用域内: {}", message_2);
    }  // 可变借用在此结束
    println!("作用域外: {}", message);  // ✅ 现在可以使用 message
    println!();
}

fn fix_method_3() {
    println!("=== 方法3: 按顺序使用 ===");

    let mut message = String::from("Hello");
    let message_2: &mut String = &mut message;
    message_2.push_str(" world");
    println!("{}", message_2);      // 先使用可变引用
    println!("{}", message);        // 可变引用使用完后，再使用原变量 ✅
    println!();
}

fn fix_method_4() {
    println!("=== 方法4: 避免冲突使用 ===");

    let message = String::from("Hello");
    let message_2 = message;  // 转移所有权，不是借用
    let mut message_4 = message_2;
    message_4.push_str(" world");
    println!("转移所有权: {}", message_4);  // ✅ 正确
    println!();
}