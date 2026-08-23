// Rust Trait 深度分析和底层实现原理

fn main() {
    println!("🎯 Rust Trait 完全演示和底层实现分析\n");

    analyze_original_code();
    demonstrate_trait_basics();
    demonstrate_vtable();
    demonstrate_generic_vs_dyn();
    demonstrate_trait_bounds();
    demonstrate_practical_examples();
}

fn analyze_original_code() {
    println!("=== 原始代码分析 ===\n");

    let person = Person {
        name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
    };

    let animal = AnimalData(String::from("Dog"));

    println!("Trait 方法调用:");
    println!("{}", person.display_info());
    println!("{}", animal.display_info());

    println!("\n默认 vs 自定义实现:");
    println!("Person 自定义: {:?}", person);
    println!("Animal 自定义: {:?}", animal);

    // 展示泛型 vs trait 对象
    log_info_2(&animal);   // 静态分发（编译时确定）
    log_info_2(&person);  // 动态分发（运行时查找 vtable）
}

fn demonstrate_trait_basics() {
    println!("=== Trait 基础演示 ===\n");

    // 1. 定义和使用 trait
    let sounder = Dog;
    Sound::make_sound(&sounder);

    let flyer = Bird;
    Sound::make_sound(&flyer);

    // 2. Trait bound
    let values = vec![1, 2, 3, 4, 5];
    let max = find_max(&values);
    println!("最大值: {}", max);

    // 3. 默认实现
    let greeter = Person {
        name: "World".to_string(),
        last_name: "Hello".to_string(),
        age: 0,
    };
    greeter.greet();

    // 4. 多个 trait bounds
    let displayable_person = Person {
        name: "Display".to_string(),
        last_name: "Person".to_string(),
        age: 30,
    };
    displayable_person.display();
    displayable_person.make_sound();
}

fn demonstrate_vtable() {
    println!("=== Trait V-Table 演示 ===\n");

    // 展示动态分发的 vtable 机制
    let dog = Dog;
    let cat = Cat;

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];

    println!("动态分发演示:");
    for animal in animals.iter() {
        animal.make_sound();
    }

    println!("\nVTable 原理:");
    println!("每个类型都有自己的 vtable");
    println!("dyn Animal 存储指向 vtable 的指针");
    println!("运行时通过 vtable 查找正确的方法实现");
}

fn demonstrate_generic_vs_dyn() {
    println!("=== 泛型 vs Trait 对象对比 ===\n");

    let dog = Dog;
    let cat = Cat;

    // 泛型版本（静态分发）
    println!("泛型版本 - 静态分发:");
    make_sound_generic(&dog);  // 编译时确定
    make_sound_generic(&cat);

    // Trait 对象版本（动态分发）
    let animal_dog: Box<dyn Animal> = Box::new(Dog);
    let animal_cat: Box<dyn Animal> = Box::new(Cat);

    println!("Trait 对象版本 - 动态分发:");
    make_sound_dyn(animal_dog);
    make_sound_dyn(animal_cat);
}

fn demonstrate_trait_bounds() {
    println!("=== Trait Bound 演示 ===\n");

    let text = "Hello World";
    let number = 42;
    let slice = vec![1, 2, 3, 4, 5];

    println!("不同 trait bound:");
    print_length(&text);              // T: Display
    is_positive(number);           // T: PartialOrd
    count_items(&slice);            // T: IntoIterator
}

fn demonstrate_practical_examples() {
    println!("=== 实用 Trait 示例 ===\n");

    // 1. Operator overloading
    demonstrate_operator_overloading();

    // 2. Iterator trait
    demonstrate_iterator_trait();

    // 3. Clone trait
    demonstrate_clone_trait();

    // 4. Drop trait
    demonstrate_drop_trait();
}

fn demonstrate_operator_overloading() {
    println!("1️⃣ 运算符重载:" );

    let point1 = Point { x: 10, y: 20 };
    let point2 = Point { x: 5, y: 15 };

    println!("Point 1: {:?}", point1);
    println!("Point 2: {:?}", point2);

    let point3 = point1 + point2;
    println!("Point 1 + Point 2: {:?}", point3);
}

fn demonstrate_iterator_trait() {
    println!("2️⃣ Iterator Trait:" );

    let my_vec = MyVector { data: vec![1, 2, 3, 4, 5], current: 0 };
    println!("向量求和: {}", my_vec.sum());

    let my_range = MyRange { start: 1, end: 5 };
    let mut range_vec = Vec::new();
    let mut range_iter = my_range;
    while let Some(val) = range_iter.next() {
        range_vec.push(val);
    }
    println!("范围迭代: {:?}", range_vec);
}

fn demonstrate_clone_trait() {
    println!("3️⃣ Clone Trait:" );

    let original = Person {
        name: "Alice".to_string(),
        last_name: "Smith".to_string(),
        age: 25,
    };

    let cloned = original.clone();  // 使用 Clone trait
    println!("原始: {:?}", original);
    println!("克隆: {:?}", cloned);
}

fn demonstrate_drop_trait() {
    println!("4️⃣ 清理 Trait:" );

    let mut resource = Resource {
        name: "数据库连接".to_string(),
    };
    println!("资源创建: {}", resource.name);
    resource.my_drop();
    println!("资源已清理");
}

// === 原始 Trait 定义 ===

trait Log {
    fn display_info(&self) -> String;
    fn alert_something(&self);
}

// === 新的演示 Trait ===

// 基础 Trait
trait Sound {
    fn make_sound(&self);
}

// Animal trait - 解决与 struct Animal 的冲突
trait Animal {
    fn make_sound(&self);
}

// Display trait - 避免与标准库冲突
trait Display {
    fn display(&self);
}

trait Summary {
    fn summarize(&self) -> String;
    fn author_info(&self) -> String;
}

// 运算符重载 Trait
trait Calculate {
    fn add(&self, other: Self) -> Self;
}

// 迭代器 Trait (自定义，避免冲突)
trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// === 结构体定义 ===

#[derive(Debug, Clone)]
struct Person {
    name: String,
    last_name: String,
    age: u32,
}

struct AnimalData(String);

#[derive(Debug)]
struct Dog;

#[derive(Debug)]
struct Cat;

#[derive(Debug)]
struct Bird;

#[derive(Debug)]
struct City(String);

// === 原始 Trait 实现和演示函数 ===

impl Log for AnimalData {
    fn display_info(&self) -> String {
        format!("Name: {}", self.0)
    }
    fn alert_something(&self) {
        println!("Alert: {}", self.0)
    }
}

impl Log for Person {
    fn display_info(&self) -> String {
        format!(
            "Name: {}, Last Name: {}, Age: {}",
            self.name, self.last_name, self.age
        )
    }
    fn alert_something(&self) {
        println!("Alert: {} {}", self.name, self.last_name)
    }
}

fn log_info<T: Log>(value: T) {
    println!("{}", value.display_info());
    value.alert_something();
}

fn log_info_2(value: &dyn Log) {
    println!("{}", value.display_info());
    value.alert_something();
}

// === 新的演示 Trait 实现 ===

impl Sound for Dog {
    fn make_sound(&self) {
        println!("汪汪！");
    }
}

impl Sound for Cat {
    fn make_sound(&self) {
        println!("喵喵！");
    }
}

impl Sound for Bird {
    fn make_sound(&self) {
        println!("叽叽喳喳！");
    }
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("动物: 汪汪！");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("动物: 喵喵！");
    }
}

impl Sound for Person {
    fn make_sound(&self) {
        println!("人说话: 你好！");
    }
}

impl Display for Person {
    fn display(&self) {
        println!("显示: {}", self.name);
    }
}

impl Summary for Person {
    fn summarize(&self) -> String {
        format!("{} 是 {} 岁", self.name, self.age)
    }

    fn author_info(&self) -> String {
        format!("作者: {}", self.name)
    }
}

// 运算符重载
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Calculate for Point {
    fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
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

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// 迭代器实现
struct MyVector {
    data: Vec<i32>,
    current: usize,
}

impl MyIterator for MyVector {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            self.current += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl MyVector {
    fn sum(&self) -> i32 {
        self.data.iter().sum()
    }
}

// 范围迭代器
struct MyRange {
    start: i32,
    end: i32,
}

impl MyIterator for MyRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let result = self.start;
            self.start += 1;
            Some(result)
        } else {
            None
        }
    }
}

// 资源管理
struct Resource {
    name: String,
}

trait MyDrop {
    fn my_drop(&mut self);
}

impl MyDrop for Resource {
    fn my_drop(&mut self) {
        println!("{} 被清理", self.name);
    }
}

// === Trait bound 函数 ===

fn print_length<T: std::fmt::Display>(value: T) {
    println!("长度: {}", value.to_string().len())
}

fn is_positive<T: std::cmp::PartialOrd + Default>(value: T) -> bool {
    value > T::default()
}

fn count_items<T: IntoIterator>(container: T) -> usize
    where
        T::Item: Copy,
{
    container.into_iter().count()
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

// 泛型函数
fn make_sound_generic<T: Animal>(animal: &T) {
    animal.make_sound();
}

fn make_sound_dyn(animal: Box<dyn Animal>) {
    animal.make_sound();
}

// === Person 其他实现 ===

impl Person {
    fn greet(&self) {
        println!("你好，我是 {}", self.name);
    }
}

// === Debug 和 Clone 实现 ===

impl std::fmt::Debug for AnimalData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AnimalData({})", self.0)
    }
}

impl Clone for AnimalData {
    fn clone(&self) -> Self {
        AnimalData(self.0.clone())
    }
}
