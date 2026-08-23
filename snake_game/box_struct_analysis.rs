// Box 和 Struct 深度分析

fn main() {
    println!("🎯 Box 和 Struct 完全演示\n");

    analyze_original_code();
    demonstrate_box_basics();
    demonstrate_struct_basics();
    demonstrate_box_vs_stack();
    demonstrate_practical_examples();
}

fn analyze_original_code() {
    println!("=== 原始代码分析 ===\n");

    // Box 使用
    let number_1 = Box::new(100);
    println!("Box 值: {}", number_1);
    println!("Box 解引用: {}", *number_1);

    // Person 结构体
    let person = Person {
        name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
    };

    let mut person2 = Person::new("name", "last_name", 10);

    println!("Person 1: {:#?}", person);
    println!("Person 2: {:#?}", person2);

    Person::some_function();
    person2.change_age(20);
    println!("修改后 Person 2: {:#?}", person2);
}

fn demonstrate_box_basics() {
    println!("=== Box 基础演示 ===\n");

    let boxed_number = Box::new(42);
    println!("Box 数字: {}", boxed_number);
    println!("解引用: {}", *boxed_number);

    use std::mem;
    println!("Box 大小: {} 字节", mem::size_of::<Box<i32>>());
    println!("i32 大小: {} 字节", mem::size_of::<i32>());
}

fn demonstrate_struct_basics() {
    println!("=== Struct 基础演示 ===\n");

    let person = SimplePerson {
        name: "Alice".to_string(),
        age: 25,
    };
    println!("简单结构体: {:?}", person);
}

fn demonstrate_box_vs_stack() {
    println!("=== Box vs 栈分配对比 ===\n");

    use std::mem;
    println!("栈上 i32 大小: {} 字节", mem::size_of::<i32>());
    println!("Box<i32> 大小: {} 字节", mem::size_of::<Box<i32>>());
}

fn demonstrate_practical_examples() {
    println!("=== 实用示例 ===\n");

    // 递归类型
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("链表: {:?}", list);

    // Trait 对象
    let dog = Dog;
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog)];
    for animal in animals.iter() {
        animal.make_sound();
    }
}

// 原始结构体
#[derive(Debug)]
struct Person {
    name: String,
    last_name: String,
    age: u32,
}

impl Person {
    fn some_function() {
        println!("Some_function");
    }

    fn from(name: String, last_name: String, age: u32) -> Self {
        Self { name, last_name, age }
    }

    fn new(name: &str, last_name: &str, age: u32) -> Self {
        Self::from(String::from(name), String::from(last_name), age)
    }

    fn change_age(&mut self, age: u32) {
        println!("Current age: {}", self.age);
        self.age = age;
        println!("Current age: {}", self.age);
    }
}

// 简化结构体
#[derive(Debug)]
struct SimplePerson {
    name: String,
    age: u32,
}

// 递归类型
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Animal trait
trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("汪汪！");
    }
}
