use std::mem;

fn main() {
    // 添加内存分析功能的增强版
    println!("=== 原始程序 + 内存分析 ===\n");

    let mut message = "Hello, world!";
    println!("原始输出: {}", message);

    // 内存分析
    println!("内存分析:");
    println!("  message 地址: {:p}", &message);
    println!("  message 大小: {} 字节", mem::size_of_val(&message));
    println!("  message 值: {:?}", message.as_ptr());

    message = "Hello, Rust!";
    println!("更新后: {}", message);
    println!("  新地址: {:p}", &message); // 地址不变，只是指针内容变了
    println!("  新值: {:?}", message.as_ptr());

    let returned_message = print_welcome(message);
    println!("返回值: {}", returned_message);
    println!("  返回值地址: {:p}", &returned_message);
    println!("\n");

    print_integer_numbers();
    print_float_numbers();
}

fn print_welcome(message: &str) -> &'static str {
    println!("函数输出: {}", message);
    println!("  参数地址: {:p}", &message);
    println!("  参数大小: {} 字节", mem::size_of_val(&message));
    "Hi there back"
}

fn print_integer_numbers() {
    println!("整数类型演示:");

    let custom_num = 90_000;
    let hex_num = 0xfa;
    let bin_num = 0b0010_1011;
    let byte_num = b'A';

    println!(
        "原始输出: {} {} {} {}",
        custom_num, hex_num, bin_num, byte_num
    );

    // 内存布局分析
    println!("内存布局:");
    println!(
        "  custom_num: 值={}, 地址={:p}, 大小={}字节",
        custom_num,
        &custom_num,
        mem::size_of_val(&custom_num)
    );
    println!(
        "  hex_num:    值={}, 地址={:p}, 大小={}字节",
        hex_num,
        &hex_num,
        mem::size_of_val(&hex_num)
    );
    println!(
        "  bin_num:    值={}, 地址={:p}, 大小={}字节",
        bin_num,
        &bin_num,
        mem::size_of_val(&bin_num)
    );
    println!(
        "  byte_num:   值={}, 地址={:p}, 大小={}字节",
        byte_num,
        &byte_num,
        mem::size_of_val(&byte_num)
    );

    // 演示栈上的内存布局
    println!("\n栈内存顺序(从高到低):");
    println!("  {:p} -> custom_num", &custom_num);
    println!("  {:p} -> hex_num", &hex_num);
    println!("  {:p} -> bin_num", &bin_num);
    println!("  {:p} -> byte_num", &byte_num);
    println!();
}

fn print_float_numbers() {
    println!("浮点和复合类型演示:");

    let float_num = 3.14;
    let float_num_2 = 3.2;
    let float_num_3 = 0.1_f32;
    let tup: (i32, &str, u8) = (20, "hello", 1);

    println!(
        "原始输出: {} {} {} {} {} {}",
        float_num, float_num_2, float_num_3, tup.0, tup.1, tup.2
    );
    println!("Debug格式: {:?}", tup);

    // 内存分析
    println!("内存分析:");
    println!(
        "  float_num:   值={}, 地址={:p}, 大小={}字节",
        float_num,
        &float_num,
        mem::size_of_val(&float_num)
    );
    println!(
        "  float_num_2: 值={}, 地址={:p}, 大小={}字节",
        float_num_2,
        &float_num_2,
        mem::size_of_val(&float_num_2)
    );
    println!(
        "  float_num_3: 值={}, 地址={:p}, 大小={}字节",
        float_num_3,
        &float_num_3,
        mem::size_of_val(&float_num_3)
    );
    println!(
        "  tuple:       地址={:p}, 总大小={}字节",
        &tup,
        mem::size_of_val(&tup)
    );

    let (a, b, c) = tup;
    println!("解构后: {} {} {}", a, b, c);
    println!("  解构变量地址: a={:p}, b={:p}, c={:p}", &a, &b, &c);

    let x: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组输出: {} {} {} {} {}", x[0], x[1], x[2], x[3], x[4]);

    let i = [2; 6];
    println!("重复数组: {:?}", i);
    println!("  x数组: 地址={:p}, 大小={}字节", &x, mem::size_of_val(&x));
    println!("  i数组: 地址={:p}, 大小={}字节", &i, mem::size_of_val(&i));

    // 字符串内存分析
    let string_literal = "hello";
    let string_heap = String::from("world");

    println!("\n字符串内存对比:");
    println!(
        "  字符串字面量: '{}' 地址={:p}, 大小={}字节",
        string_literal,
        &string_literal,
        mem::size_of_val(&string_literal)
    );
    println!(
        "  String堆分配: '{}' 地址={:p}, 大小={}字节",
        string_heap,
        &string_heap,
        mem::size_of_val(&string_heap)
    );
    println!("    堆数据地址: {:?}", string_heap.as_ptr());
    println!("    容量: {} 字节", string_heap.capacity());
}
