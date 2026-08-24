fn main() {
    println!("🎯 多文件模块系统演示\n");

    demonstrate_animal_modules();
    demonstrate_math_modules();
    demonstrate_module_paths();
    demonstrate_file_organization();
}

fn demonstrate_animal_modules() {
    println!("=== 动物模块演示 ===\n");

    // 使用完整的模块路径
    let my_dog = snake_game::module_demo::animals::dog::Dog::new("旺财".to_string(), 3);
    my_dog.bark();
    my_dog.sleep();

    let my_cat = snake_game::module_demo::animals::cat::Cat::new("咪咪".to_string(), "橘色".to_string());
    my_cat.meow();

    println!("\n使用自由函数:");
    let another_dog = snake_game::module_demo::animals::dog::create_dog("大黄", 5);
    another_dog.bark();
}

fn demonstrate_math_modules() {
    println!("\n=== 数学模块演示 ===\n");

    println!("基础运算:");
    println!("5 + 3 = {}", snake_game::module_demo::math::calculator::add(5, 3));
    println!("10 - 4 = {}", snake_game::module_demo::math::calculator::subtract(10, 4));

    // 几何计算
    let rect = snake_game::module_demo::math::geometry::Rectangle::new(5.0, 3.0);
    println!("矩形面积: {}", rect.area());
}

fn demonstrate_module_paths() {
    println!("\n=== 模块路径演示 ===\n");

    snake_game::module_demo::demo_info();
    
    use snake_game::module_demo::{animals, math};
    animals::animal_info();
    math::math_info();
}

fn demonstrate_file_organization() {
    println!("\n=== 文件组织演示 ===\n");
    println!("📁 模块化文件结构:");
    println!("src/");
    println!("├── lib.rs              # 库入口");
    println!("├── module_demo/        # 演示模块");
    println!("│   ├── mod.rs");
    println!("│   ├── animals/");
    println!("│   │   ├── mod.rs");
    println!("│   │   ├── dog.rs");
    println!("│   │   └── cat.rs");
    println!("│   └── math/");
    println!("│       ├── mod.rs");
    println!("│       ├── calculator.rs");
    println!("│       └── geometry.rs");

    println!("\n💡 优势:");
    println!("✓ 代码按功能分类");
    println!("✓ 便于查找和维护");
    println!("✓ 支持独立测试");

    println!("\n🔑 关键概念:");
    println!("mod → 定义模块");
    println!("pub → 公开访问");
    println!("use → 导入路径");
}
