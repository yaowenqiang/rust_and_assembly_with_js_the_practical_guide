// Rust Trait 简单易懂的示例

fn main() {
    println!("🎯 Rust Trait 简单演示\n");

    basic_trait_examples();
    vtable_examples();
    static_vs_dynamic_examples();
    trait_bounds_examples();
    practical_examples();
}

fn basic_trait_examples() {
    println!("=== Trait 基础示例 ===\n");

    // 1. 定义和使用 trait
    let dog = Dog;
    let cat = Cat;

    dog.make_sound();
    cat.make_sound();

    // 2. Trait 对象
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    println!("动物叫声:");
    for animal in animals.iter() {
        animal.make_sound();
    }
}

fn vtable_examples() {
    println!("=== VTable 演示 ===\n");

    use std::mem;

    // 胖指针大小演示
    let dog = Dog;
    let cat = Cat;

    println!("Trait 对象大小: {} 字节", mem::size_of::<&dyn Animal>());
    println!("Box<dyn Animal> 大小: {} 字节", mem::size_of::<Box<dyn Animal>>());

    println!("\nVTable 工作原理:");
    println!("1. 每个 Dog 实例都有指向 Dog vtable 的指针");
    println!("2. 调用 make_sound 时通过 vtable 找到 Dog::make_sound");
    println!("3. 间接调用，但有运行时多态的灵活性");
}

fn static_vs_dynamic_examples() {
    println!("=== 静态分发 vs 动态分发 ===\n");

    let dog = Dog;
    let cat = Cat;

    // 静态分发 - 泛型
    println!("静态分发 (泛型):");
    generic_make_sound(&dog);  // 编译时确定为 Dog::make_sound
    generic_make_sound(&cat);  // 编译时确定为 Cat::make_sound

    // 动态分发 - Trait 对象
    println!("动态分发 (Trait 对象):");
    dynamic_make_sound(Box::new(dog));  // 运行时查找 vtable
    dynamic_make_sound(Box::new(cat));  // 运行时查找 vtable
}

fn trait_bounds_examples() {
    println!("=== Trait Bound 示例 ===\n");

    // 1. 基础 Trait Bound
    let text = "Hello";
    let number = 42;

    print_info(text);
    print_info(number);

    // 2. 多个 Trait Bounds
    let point = Point { x: 10, y: 20 };
    display_and_compare(point, Point { x: 5, y: 15 });

    // 3. Where 子句
    let values = vec![1, 2, 3, 4, 5];
    let max = find_max(&values);
    println!("最大值: {}", max);
}

fn practical_examples() {
    println!("=== 实用 Trait 示例 ===\n");

    // 1. Clone Trait
    clone_example();

    // 2. Display Trait
    display_example();

    // 3. Default Trait
    default_example();

    // 4. 运算符重载
    operator_example();
}

fn clone_example() {
    println!("1️⃣ Clone Trait:");

    let original = Person {
        name: "Alice".to_string(),
        age: 25,
    };

    let cloned = original.clone();
    println!("原始: {:?}", original);
    println!("克隆: {:?}", cloned);
}

fn display_example() {
    println!("2️⃣ Display Trait:");

    let person = Person {
        name: "Bob".to_string(),
        age: 30,
    };

    println!("显示: {}", person);
}

fn default_example() {
    println!("3️⃣ Default Trait:");

    let default_person = Person::default();
    println!("默认值: {:?}", default_person);
}

fn operator_example() {
    println!("4️⃣ 运算符重载:");

    let point1 = Point { x: 10, y: 20 };
    let point2 = Point { x: 5, y: 15 };

    let sum = point1 + point2;
    println!("{} + {} = {:?}", point1, point2, sum);
}

// === Trait 定义 ===

trait Animal {
    fn make_sound(&self);
}

trait Display {
    fn display(&self) -> String;
}

// === 结构体定义 ===

#[derive(Debug, Clone)]
struct Dog;

#[derive(Debug, Clone)]
struct Cat;

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

// === Trait 实现 ===

impl Animal for Dog {
    fn make_sound(&self) {
        println!("汪汪！");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("喵喵！");
    }
}

impl Display for Person {
    fn display(&self) -> String {
        format!("{} ({}岁)", self.name, self.age)
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({}岁)", self.name, self.age)
    }
}

impl std::default::Default for Person {
    fn default() -> Self {
        Person {
            name: "Unknown".to_string(),
            age: 0,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

// === 泛型函数 (静态分发) ===

fn generic_make_sound<T: Animal>(animal: &T) {
    animal.make_sound();
}

fn dynamic_make_sound(animal: Box<dyn Animal>) {
    animal.make_sound();
}

// === Trait Bound 函数 ===

fn print_info<T: std::fmt::Display>(value: T) {
    println!("信息: {}", value);
}

fn display_and_compare<T: std::fmt::Display + std::cmp::PartialOrd>(
    a: T,
    b: T,
) {
    println!("{} vs {}", a, b);
    if a > b {
        println!("{} 更大", a);
    } else {
        println!("{} 更大或相等", b);
    }
}

fn find_max<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list.iter().skip(1) {
        if item > max {
            max = item;
        }
    }
    max
}