// Box 和 Struct 简单易懂的示例

fn main() {
    println!("🎯 Box 和 Struct 简单演示\n");

    demonstrate_box_basics();
    demonstrate_struct_basics();
    demonstrate_box_vs_stack();
    demonstrate_practical_examples();
}

fn demonstrate_box_basics() {
    println!("=== Box 基础演示 ===\n");

    // 1. Box 创建和访问
    let boxed_number = Box::new(42);
    println!("Box 数字: {}", boxed_number);
    println!("解引用: {}", *boxed_number);

    // 2. 不同类型的 Box
    let boxed_string = Box::new(String::from("Hello"));
    println!("Box 字符串: {}", boxed_string);

    let boxed_array = Box::new([1, 2, 3, 4, 5]);
    println!("Box 数组: {:?}", boxed_array);

    // 3. 内存大小对比
    use std::mem;
    println!("\n内存大小对比:");
    println!("i32 大小: {} 字节", mem::size_of::<i32>());
    println!("Box<i32> 大小: {} 字节", mem::size_of::<Box<i32>>());

    let stack_array = [1, 2, 3];
    let boxed_array = Box::new([1, 2, 3]);
    println!("栈上数组大小: {} 字节", mem::size_of::<[i32; 3]>());
    println!("Box<[i32; 3]> 大小: {} 字节", mem::size_of::<Box<[i32; 3]>>());
}

fn demonstrate_struct_basics() {
    println!("=== Struct 基础演示 ===\n");

    // 1. 结构体创建
    let person = SimplePerson {
        name: "Alice".to_string(),
        age: 25,
    };
    println!("结构体: {:?}", person);

    // 2. 可变结构体
    let mut person = SimplePerson {
        name: "Bob".to_string(),
        age: 30,
    };
    println!("修改前: {:?}", person);
    person.age = 31;
    println!("修改后: {:?}", person);

    // 3. 结构体方法
    person.greet();
    person.birthday();

    // 4. 关联函数
    let person2 = SimplePerson::new("Charlie", 35);
    println!("关联函数创建: {:?}", person2);
}

fn demonstrate_box_vs_stack() {
    println!("=== Box vs 栈分配对比 ===\n");

    use std::mem;

    // 小数据类型
    let small_stack = 42;
    let small_heap = Box::new(42);

    println!("小数据类型:");
    println!("栈上 i32: {}", small_stack);
    println!("堆上 i32: {}", small_heap);
    println!("栈大小: {} 字节", mem::size_of::<i32>());
    println!("Box 大小: {} 字节", mem::size_of::<Box<i32>>());

    // 大数据类型
    let large_stack = [0u8; 1000];    // 1000 字节数组在栈上
    let large_heap = Box::new([0u8; 1000]); // 1000 字节数组在堆上

    println!("\n大数据类型:");
    println!("栈上数组大小: {} 字节", mem::size_of::<[u8; 1000]>());
    println!("堆上数组大小: {} 字节", mem::size_of::<Box<[u8; 1000]>>());

    // 地址对比
    println!("\n地址对比:");
    println!("栈值地址: {:p}", &small_stack);
    println!("Box 自身地址: {:p}", &small_heap);
    println!("Box 指向地址: {:p}", small_heap.as_ref());
}

fn demonstrate_practical_examples() {
    println!("=== 实用 Box 和 Struct 示例 ===\n");

    // 1. 动态大小的数据
    dynamic_sized_data();

    // 2. 转移大数据
    large_data_transfer();

    // 3. 隐藏实现细节
    encapsulation_example();

    // 4. 多态示例
    polymorphism_example();
}

fn dynamic_sized_data() {
    println!("1️⃣ 动态大小数据:");

    // Vec 是动态大小的，适合用 Box
    let vec_data = Box::new(vec![1, 2, 3, 4, 5]);
    println!("Box Vec: {:?}", vec_data);

    // 字符串也可以 Box
    let boxed_string = Box::new(String::from("Dynamic sized"));
    println!("Box String: {}", boxed_string);
}

fn large_data_transfer() {
    println!("2️⃣ 大数据转移优势:");

    let large_data = Box::new(vec![1u8; 10000]); // 10KB 数据
    println!("转移前数据长度: {}", large_data.len());

    // 转移所有权 - 只复制指针（8字节），不复制10KB数据
    let transferred_data = large_data;
    println!("转移后数据长度: {}", transferred_data.len());
}

fn encapsulation_example() {
    println!("3️⃣ 封装示例:");

    let secret = SecretStruct::new("hidden value");
    println!("加密数据: {}", secret.get_public());
    // secret.private_field;  // 编译错误！字段是私有的
}

fn polymorphism_example() {
    println!("4️⃣ 多态示例:");

    let dog = Dog;
    let cat = Cat;

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];

    for animal in animals.iter() {
        animal.make_sound();
    }
}

// === 结构体定义 ===

#[derive(Debug)]
struct SimplePerson {
    name: String,
    age: u32,
}

impl SimplePerson {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn greet(&self) {
        println!("你好，我是 {}", self.name);
    }

    fn birthday(&mut self) {
        self.age += 1;
        println!("生日快乐！现在 {} 岁", self.age);
    }
}

// 封装结构体
struct SecretStruct {
    private_field: String,
}

impl SecretStruct {
    fn new(value: &str) -> Self {
        Self {
            private_field: value.to_string(),
        }
    }

    fn get_public(&self) -> String {
        format!("***{}***", self.private_field)
    }
}

// Trait 和多态
trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("汪汪！");
    }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) {
        println!("喵喵！");
    }
}
