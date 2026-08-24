// Rust 模块系统完全演示
// 编译命令：rustc module_system_examples.rs -o module_system_examples

fn main() {
    println!("🎯 Rust 模块系统完全演示\n");

    basic_module_examples();
    visibility_examples();
    use_examples();
    practical_examples();
}

fn basic_module_examples() {
    println!("=== 模块基础演示 ===\n");

    // 1. 内联模块
    println!("1️⃣ 内联模块:");
    animals::dog::bark();
    animals::cat::meow();

    // 2. 嵌套模块
    println!("\n2️⃣ 嵌套模块:");
    company::engineering::backend::serve();
    company::engineering::frontend::render();
}

fn visibility_examples() {
    println!("\n=== 可见性演示 ===\n");

    // 1. 公共函数可以访问
    println!("1️⃣ 公共函数:");
    visibility::public_function();

    // 2. 私有函数通过公共接口访问
    println!("\n2️⃣ 私有函数（通过公共接口访问）:");
    visibility::call_private();
}

fn use_examples() {
    println!("\n=== Use 语句演示 ===\n");

    println!("1️⃣ 基础 use:");
    demonstrate_basic_use();

    println!("\n2️⃣ Use 的各种形式:");
    demonstrate_use_forms();
}

fn practical_examples() {
    println!("\n=== 实用模块组织示例 ===\n");

    println!("1️⃣ 库 vs 二进制项目:");
    demonstrate_lib_binary_organization();

    println!("\n2️⃣ 文件结构推荐:");
    demonstrate_file_structure();
}

// === 演示函数 ===

fn demonstrate_basic_use() {
    // 使用 use 简化路径
    use animals::dog;
    use animals::cat;

    println!("  使用 use 简化调用:");
    dog::bark();
    cat::meow();
}

fn demonstrate_use_forms() {
    // 单个导入
    use animals::dog::bark;
    bark();
    
    // 多个导入（不重复）
    use animals::cat::meow;
    meow();
    
    // 同时导入多个
    use animals::{dog::sleep, cat::scratch};
    sleep();
    scratch();
    
    println!("  ✓ 单个导入: use animals::dog::bark");
    println!("  ✓ 多个导入: use animals::{{dog::bark, cat::meow}}");
}

fn demonstrate_lib_binary_organization() {
    println!("  库项目 (lib.rs): 可被其他项目依赖");
    println!("  二进制项目 (main.rs): 独立可执行程序");
}

fn demonstrate_file_structure() {
    println!("  推荐文件结构:");
    println!("  src/");
    println!("  ├── main.rs");
    println!("  ├── lib.rs");
    println!("  ├── animals/");
    println!("  │   ├── mod.rs");
    println!("  │   ├── dog.rs");
    println!("  │   └── cat.rs");
}

// === 内联模块定义 ===

mod animals {
    pub mod dog {
        pub fn bark() {
            println!("    🐕 汪汪！");
        }
        
        pub fn sleep() {
            println!("    🐕 狗狗在睡觉...");
        }
    }

    pub mod cat {
        pub fn meow() {
            println!("    🐱 喵喵！");
        }
        
        pub fn scratch() {
            println!("    🐱 猫咪在抓挠...");
        }
    }
}

mod company {
    pub mod engineering {
        pub mod backend {
            pub fn serve() {
                println!("    🖥️ 后端服务运行中...");
            }
        }

        pub mod frontend {
            pub fn render() {
                println!("    🎨 前端界面渲染中...");
            }
        }
    }
}

mod visibility {
    pub fn public_function() {
        println!("    🔓 这是公共函数");
    }

    fn private_function() {
        println!("    🔒 这是私有函数");
    }

    pub fn call_private() {
        println!("    通过公共接口调用私有函数:");
        private_function();
    }
}
