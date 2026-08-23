// Rust 元编程示例：获取 Struct 名称

// === 自定义宏定义 (必须在调用前定义) ===

macro_rules! show_type_name {
    ($value:expr) => {
        println!("{} 的类型是: {}", stringify!($value), std::any::type_name_of_val(&$value));
    };
}

macro_rules! impl_describe {
    ($type:ty) => {
        impl $type {
            fn describe(&self) {
                println!("我是 {} 类型", std::any::type_name::<Self>());
            }
        }
    };
}

fn main() {
    println!("🎯 Rust 元编程：获取 Struct 名称\n");

    basic_type_name_examples();
    any_trait_examples();
    macro_examples();
    custom_trait_examples();
    procedural_macro_examples();
}

fn basic_type_name_examples() {
    println!("=== 基础类型名称获取 ===\n");

    // 使用 std::any::type_name
    let dog = Dog;
    let cat = Cat;
    let person = Person {
        name: "Alice".to_string(),
        age: 25,
    };

    dog.show_type();
    cat.show_type();
    person.show_type();

    println!("直接调用 type_name:");
    println!("Dog 类型: {}", std::any::type_name::<Dog>());
    println!("Cat 类型: {}", std::any::type_name::<Cat>());
    println!("Person 类型: {}", std::any::type_name::<Person>());
}

fn any_trait_examples() {
    println!("\n=== Any Trait 运行时类型识别 ===\n");

    let dog = Dog;
    let cat = Cat;
    let person = Person {
        name: "Bob".to_string(),
        age: 30,
    };

    // 使用 Any trait 进行运行时类型检查
    check_type_any(&dog);
    check_type_any(&cat);
    check_type_any(&person);

    // 类型转换示例
    let animal: Box<dyn Animal> = Box::new(Dog);
    animal.as_any().downcast_ref::<Dog>().map(|d| d.make_sound());
}

fn macro_examples() {
    println!("\n=== 宏：编译时元编程 ===\n");

    let dog = Dog;
    let cat = Cat;
    let person = Person {
        name: "Charlie".to_string(),
        age: 35,
    };

    // 使用自定义宏显示类型名称
    show_type_name!(dog);
    show_type_name!(cat);
    show_type_name!(person);

    // 使用宏实现 trait
    dog.describe();
    cat.describe();
    person.describe();
}

fn custom_trait_examples() {
    println!("\n=== 自定义 Trait：类型名称获取 ===\n");

    let dog = Dog;
    let cat = Cat;
    let person = Person {
        name: "David".to_string(),
        age: 40,
    };

    dog.type_info();
    cat.type_info();
    person.type_info();

    println!("\n带类型的泛型函数:");
    print_type_info(&dog);
    print_type_info(&cat);
    print_type_info(&person);
}

fn procedural_macro_examples() {
    println!("\n=== 过程宏（模拟）示例 ===\n");

    println!("过程宏可以在编译时生成代码:");
    println!("1. 派生宏 (Derive Macros) - 自动实现 trait");
    println!("2. 属性宏 (Attribute Macros) - 修改函数/结构体");
    println!("3. 函数宏 (Function-like Macros) - 自定义语法");

    // 模拟使用 derive 宏
    let named_struct = NamedStruct {
        name: "Example".to_string(),
        value: 42,
    };

    named_struct.show_name();
    println!("自动生成的实现: {}", named_struct.get_name());
}

// === 基础 Trait 定义 ===

trait Animal {
    fn make_sound(&self);
    fn show_type(&self);
    fn as_any(&self) -> &dyn std::any::Any;
}

trait TypeName {
    fn type_name(&self) -> &'static str;
    fn type_info(&self);
}

// === 结构体定义 ===

#[derive(Debug)]
struct Dog;

#[derive(Debug)]
struct Cat;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
struct NamedStruct {
    name: String,
    value: i32,
}

// === Animal Trait 实现 ===

impl Animal for Dog {
    fn make_sound(&self) {
        println!("汪汪！");
    }

    fn show_type(&self) {
        println!("我是: {}", std::any::type_name::<Dog>());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("喵喵！");
    }

    fn show_type(&self) {
        println!("我是: {}", std::any::type_name::<Cat>());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Animal for Person {
    fn make_sound(&self) {
        println!("人说话: 你好！");
    }

    fn show_type(&self) {
        println!("我是: {}", std::any::type_name::<Person>());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// === TypeName Trait 实现 ===

impl TypeName for Dog {
    fn type_name(&self) -> &'static str {
        "Dog"
    }

    fn type_info(&self) {
        println!("类型: Dog, 大小: {} 字节", std::mem::size_of::<Dog>());
    }
}

impl TypeName for Cat {
    fn type_name(&self) -> &'static str {
        "Cat"
    }

    fn type_info(&self) {
        println!("类型: Cat, 大小: {} 字节", std::mem::size_of::<Cat>());
    }
}

impl TypeName for Person {
    fn type_name(&self) -> &'static str {
        "Person"
    }

    fn type_info(&self) {
        println!("类型: Person, 大小: {} 字节", std::mem::size_of::<Person>());
    }
}

// === 辅助函数 ===

fn check_type_any(value: &dyn Animal) {
    value.as_any().downcast_ref::<Dog>().map_or_else(
        || {
            value.as_any().downcast_ref::<Cat>().map_or_else(
                || {
                    value.as_any().downcast_ref::<Person>().map_or_else(
                        || println!("未知类型"),
                        |p| println!("这是 Person: {}", p.name),
                    )
                },
                |_| println!("这是 Cat"),
            )
        },
        |_| println!("这是 Dog"),
    )
}

fn print_type_info<T: TypeName>(value: &T) {
    println!("泛型类型信息: {}", value.type_name());
    value.type_info();
}

// === 为类型应用宏 ===
impl_describe!(Dog);
impl_describe!(Cat);
impl_describe!(Person);

// === NamedStruct 自动实现 (模拟派生宏) ===

impl NamedStruct {
    fn show_name(&self) {
        println!("Struct 名称: {}", std::any::type_name::<Self>());
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
