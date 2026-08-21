use std::mem;

fn main() {
    // 演示如何查看内存布局和大小

    // 1. 查看基本数据类型的大小
    println!("=== 基本类型大小 ===");
    println!("i32 大小: {} 字节", mem::size_of::<i32>());
    println!("f64 大小: {} 字节", mem::size_of::<f64>());
    println!("&str 大小: {} 字节", mem::size_of::<&str>());
    println!("bool 大小: {} 字节", mem::size_of::<bool>());

    // 2. 查看变量的内存地址
    let a = 10;
    let b = 20;
    let c = 30;

    println!("\n=== 变量内存地址 ===");
    println!("a: 值={}, 地址={:p}, 大小={} 字节", a, &a, mem::size_of_val(&a));
    println!("b: 值={}, 地址={:p}, 大小={} 字节", b, &b, mem::size_of_val(&b));
    println!("c: 值={}, 地址={:p}, 大小={} 字节", c, &c, mem::size_of_val(&c));

    // 3. 复合类型的内存布局
    let tuple: (i32, f64, bool) = (42, 3.14, true);
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("\n=== 复合类型内存 ===");
    println!("tuple 大小: {} 字节", mem::size_of_val(&tuple));
    println!("tuple 地址: {:p}", &tuple);
    println!("array 大小: {} 字节", mem::size_of_val(&array));
    println!("array 地址: {:p}", &array);

    // 4. 堆内存分配
    let heap_value = Box::new(1000);
    let heap_string = String::from("Hello, heap memory!");

    println!("\n=== 堆内存 ===");
    println!("Box<i32> 栈上大小: {} 字节", mem::size_of::<Box<i32>>());
    println!("Box 值地址: {:p}", heap_value);
    println!("String 栈上大小: {} 字节", mem::size_of::<String>());
    println!("String 地址: {:p}", &heap_string);
    println!("String 长度: {} 字节", heap_string.len());

    // 5. 结构体内存对齐演示
    struct AlignDemo {
        a: u8,
        b: u32,
        c: u8,
    }

    let align_demo = AlignDemo { a: 1, b: 2, c: 3 };
    println!("\n=== 内存对齐 ===");
    println!("AlignDemo 总大小: {} 字节", mem::size_of_val(&align_demo));
    println!("  a: {} 字节", mem::size_of_val(&align_demo.a));
    println!("  b: {} 字节", mem::size_of_val(&align_demo.b));
    println!("  c: {} 字节", mem::size_of_val(&align_demo.c));

    // 6. Vec 内存布局
    let vec = vec![1, 2, 3, 4, 5];
    println!("\n=== Vec 内存布局 ===");
    println!("Vec 栈上大小: {} 字节", mem::size_of::<Vec<i32>>());
    println!("Vec 地址: {:p}", &vec);
    println!("Vec 容量: {}", vec.capacity());
    println!("Vec 长度: {}", vec.len());
    println!("Vec 堆内存大小: {} 字节", vec.capacity() * mem::size_of::<i32>());
}