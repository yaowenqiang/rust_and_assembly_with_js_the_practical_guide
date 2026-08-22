// Rust 可变性和 Copy 语义深度分析

fn main() {
    println!("=== 可变性独立性演示 ===\n");

    // === 1. 基本情况：不可变变量传递给可变参数 ===
    let b = 10;                    // b 是不可变的 (immutable)
    println!("调用前 b = {}", b);
    extend_age(b);                 // ✅ 可以传递！
    println!("调用后 b = {}\n", b); // b 仍然是 10，且仍然是不可变的

    // === 2. 可变变量传递给不可变参数 ===
    let mut c = 20;                // c 是可变的 (mutable)
    println!("调用前 c = {}", c);
    print_value(c);                // ✅ 可以传递！
    println!("调用后 c = {}\n", c); // c 仍然是 20，且仍然是可变的

    // === 3. 演示函数参数是独立的变量 ===
    let x = 100;
    println!("调用前 x = {}", x);
    demonstrate_independence(x);
    println!("调用后 x = {}\n", x); // x 不受影响

    // === 4. 对比：可变引用的情况 ===
    let mut d = 30;
    println!("调用前 d = {}", d);
    extend_age_ref(&mut d);        // 需要传递可变引用
    println!("调用后 d = {}\n", d); // d 被修改了！

    // === 5. 错误情况演示（注释掉，无法编译） ===
    /*
    let e = 40;
    extend_age_ref(&mut e);         // ❌ 编译错误！e 不是可变的
    */

    // === 6. 内存地址演示 ===
    demonstrate_memory_addresses();
}

// 原始函数：参数是可变的
fn extend_age(mut a: u32) {
    println!("  函数内开始: a = {}", a);
    a += 100;                       // a 是函数内的可变变量
    println!("  函数内修改: a = {}", a);
    // 函数结束，a 被销毁
}

// 参数是不可变的
fn print_value(a: u32) {
    println!("  函数内接收到: a = {}", a);
    // a += 100;                    // ❌ 编译错误！a 是不可变的
}

// 演示独立性
fn demonstrate_independence(mut a: u32) {
    println!("  函数内开始: a = {}", a);
    a *= 2;                         // 修改函数内的副本
    println!("  函数内修改: a = {}", a);
    println!("  函数内 a 的地址: {:p}", &a);
}

// 使用可变引用：这会真正修改原变量
fn extend_age_ref(a: &mut u32) {
    println!("  函数内接收到: a = {}", a);
    *a += 100;                      // 解引用并修改原值
    println!("  函数内修改: a = {}", a);
}

// 演示内存地址
fn demonstrate_memory_addresses() {
    println!("=== 内存地址独立性演示 ===\n");

    let original = 42;
    println!("original 的地址: {:p}", &original);
    println!("original 的值: {}", original);

    show_memory_address(original);
    println!("调用后 original 的地址: {:p}", &original);
    println!("调用后 original 的值: {}\n", original);
}

fn show_memory_address(mut a: u32) {
    println!("函数内 a 的地址: {:p}", &a);
    println!("函数内 a 的值: {}", a);
    a *= 10;
    println!("函数内修改后 a 的值: {}", a);
}