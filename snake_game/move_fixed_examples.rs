// Rust 所有权和移动语义修复示例

// === 方法1: 返回修改后的值 (推荐) ===
fn extend_age_return(mut a: u32) -> u32 {
    a += 100;
    a  // 返回修改后的值
}

fn demo_return_method() {
    let mut b = 10;  // 添加 mut
    println!("原始值: {}", b);
    b = extend_age_return(b);  // 接收返回值
    println!("修改后: {}", b);  // 110
}

// === 方法2: 使用可变引用 ===
fn extend_age_ref(a: &mut u32) {
    *a += 100;  // 解引用并修改原值
}

fn demo_ref_method() {
    let mut b = 10;
    println!("原始值: {}", b);
    extend_age_ref(&mut b);  // 传递可变引用
    println!("修改后: {}", b);  // 110
}

// === 方法3: 对比 Copy vs Move ===
fn demo_copy_vs_move() {
    println!("\n=== Copy vs Move 对比 ===");

    // u32: Copy 类型
    let x = 42;
    let y = x;              // x 被拷贝，x 仍然可用
    println!("x: {}, y: {}", x, y);  // 两个都可以用

    // String: Move 类型
    let s1 = String::from("Hello");
    let s2 = s1;            // s1 被移动到 s2，s1 不再可用
    // println!("{}", s1);  // ❌ 编译错误：s1 被移动了
    println!("s2: {}", s2);  // ✅ 只有 s2 可用
}

// === 方法4: 演示函数参数的 Copy ===
fn takes_u32(mut a: u32) {  // 添加 mut
    println!("函数内接收到: {}", a);
    a += 1000;              // 只修改函数内的副本
    println!("函数内修改后: {}", a);
}

fn demo_function_copy() {
    let original = 42;
    println!("调用函数前: {}", original);
    takes_u32(original);      // 传递副本
    println!("调用函数后: {}", original);  // 原值不变
}

// === 方法5: 演示函数参数的 Move ===
fn takes_string(mut s: String) {  // 添加 mut
    println!("函数内接收到: {}", s);
    s.push_str(" World");
    println!("函数内修改后: {}", s);
    // s 在这里被销毁
}

fn demo_function_move() {
    let original = String::from("Hello");
    println!("调用函数前: {}", original);
    takes_string(original);  // ← 所有权转移！
    // println!("{}", original);  // ❌ 编译错误：original 被移动了
}

// === 方法6: String 的正确使用方式 ===
fn modify_string_return(s: String) -> String {
    let mut result = s;
    result.push_str(" World");
    result  // 返回修改后的所有权
}

fn demo_string_correct() {
    let mut message = String::from("Hello");  // 添加 mut
    println!("修改前: {}", message);
    message = modify_string_return(message);  // 重新接收所有权
    println!("修改后: {}", message);  // "Hello World"
}

// === 方法7: 引用避免移动 ===
fn modify_string_ref(s: &mut String) {
    s.push_str(" World");  // 通过引用修改，不转移所有权
}

fn demo_string_ref() {
    let mut message = String::from("Hello");
    println!("修改前: {}", message);
    modify_string_ref(&mut message);  // 传递可变引用
    println!("修改后: {}", message);  // "Hello World"
    // message 仍然可用！
}

fn main() {
    println!("=== 方法1: 返回值 ===");
    demo_return_method();

    println!("\n=== 方法2: 可变引用 ===");
    demo_ref_method();

    demo_copy_vs_move();

    println!("\n=== 函数参数 Copy 演示 ===");
    demo_function_copy();

    println!("\n=== 函数参数 Move 演示 ===");
    demo_function_move();

    println!("\n=== String 正确使用 ===");
    demo_string_correct();

    println!("\n=== String 引用使用 ===");
    demo_string_ref();
}