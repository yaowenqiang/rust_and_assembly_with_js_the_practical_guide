// Rust Enum 简单易懂的示例

fn main() {
    println!("🎯 Rust Enum 简单演示\n");

    basic_enum_examples();
    memory_model_examples();
    pattern_matching_examples();
    practical_examples();
}

fn basic_enum_examples() {
    println!("=== Enum 基础示例 ===\n");

    // 1. 简单枚举（无数据）
    let direction = Direction::North;
    println!("方向: {:?}", direction);

    // 2. 带数据的枚举
    let status = Status::Success(200);
    println!("状态: {:?}", status);

    // 3. 多字段枚举
    let message = Message::Move { x: 10, y: 20 };
    println!("消息: {:?}", message);
}

fn memory_model_examples() {
    println!("=== Enum 内存模型 ===\n");

    use std::mem;

    // C 风格枚举 - 最小内存占用
    println!("C 风格枚举大小:");
    println!("Direction: {} 字节", mem::size_of::<Direction>());
    println!("Color: {} 字节", mem::size_of::<Color>());

    // 带数据枚举 - 最大变体大小
    println!("\n带数据枚举大小:");
    println!("Status: {} 字节", mem::size_of::<Status>());

    let success = Status::Success(200);
    let error = Status::Error(404);
    println!("Success 实例: {} 字节", mem::size_of_val(&success));
    println!("Error 实例: {} 字节", mem::size_of_val(&error));

    // Option 枚举
    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;
    println!("\nOption 大小:");
    println!("Some(i32): {} 字节", mem::size_of_val(&some_value));
    println!("None: {} 字节", mem::size_of_val(&no_value));
    println!("Option<i32>: {} 字节", mem::size_of::<Option<i32>>());
}

fn pattern_matching_examples() {
    println!("=== 模式匹配示例 ===\n");

    // 1. 匹配枚举值
    let direction = Direction::South;
    match direction {
        Direction::North => println!("向北走"),
        Direction::South => println!("向南走"),
        Direction::East => println!("向东走"),
        Direction::West => println!("向西走"),
    }

    // 2. 匹配并提取数据
    let status = Status::Error(404);
    match status {
        Status::Success(code) => println!("成功: {}", code),
        Status::Error(code) => println!("错误: {}", code),
    }

    // 3. 忽略数据的匹配
    let status = Status::Success(200);
    match status {
        Status::Success(_) => println!("成功（忽略代码）"),
        Status::Error(_) => println!("错误（忽略代码）"),
    }

    // 4. if let 匹配
    let status = Status::Success(200);
    if let Status::Success(code) = status {
        println!("if let 成功: {}", code);
    }
}

fn practical_examples() {
    println!("=== 实用 Enum 示例 ===\n");

    // 1. Option - 处理可能不存在的值
    option_example();

    // 2. Result - 处理可能失败的操作
    result_example();

    // 3. 状态机
    state_machine_example();

    // 4. 错误类型
    error_handling_example();
}

fn option_example() {
    println!("1️⃣ Option - 处理可能不存在的值:");

    let found = Some(42);
    let not_found: Option<i32> = None;

    println!("Found: {:?}", found);
    println!("Not found: {:?}", not_found);

    // 实际使用
    let number = Some(10);
    match number {
        Some(value) => println!("值是: {}", value),
        None => println!("无值"),
    }
}

fn result_example() {
    println!("2️⃣ Result - 处理可能失败的操作:");

    let success = AppResult::Success(1000);
    let failure = AppResult::Error("连接失败");

    println!("成功: {:?}", success);
    println!("失败: {:?}", failure);

    match success {
        AppResult::Success(value) => println!("操作成功，返回: {}", value),
        AppResult::Error(msg) => println!("操作失败: {}", msg),
    }
}

fn state_machine_example() {
    println!("3️⃣ 状态机 - 枚举表示状态:");

    let mut switch = SwitchState::Off;
    println!("初始状态: {:?}", switch);

    switch.toggle();
    println!("切换后: {:?}", switch);

    switch.toggle();
    println!("再次切换: {:?}", switch);
}

impl SwitchState {
    fn toggle(&mut self) {
        *self = match self {
            SwitchState::Off => SwitchState::On,
            SwitchState::On => SwitchState::Off,
        };
    }
}

fn error_handling_example() {
    println!("4️⃣ 错误处理 - 枚举表示错误类型:");

    let file_error = FileSystemError::FileNotFound {
        filename: "data.txt".to_string()
    };
    
    let permission_error = FileSystemError::PermissionDenied {
        operation: "write".to_string()
    };

    match file_error {
        FileSystemError::FileNotFound { filename } => {
            println!("文件未找到: {}", filename);
        }
        FileSystemError::PermissionDenied { operation } => {
            println!("权限被拒绝: {}", operation);
        }
    }

    match permission_error {
        FileSystemError::FileNotFound { .. } => println!("文件错误"),
        FileSystemError::PermissionDenied { operation } => {
            println!("权限错误: {}", operation);
        }
    }
}

// === 基础枚举定义 ===

// C 风格枚举（无数据）
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

// 带数据的枚举
#[derive(Debug)]
enum Status {
    Success(u32),
    Error(u32),
}

// 多字段枚举
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor { r: u8, g: u8, b: u8 },
}

// Result 枚举
#[derive(Debug)]
enum AppResult {
    Success(i32),
    Error(&'static str),
}

// 状态机枚举
#[derive(Debug)]
enum SwitchState {
    On,
    Off,
}

// 错误类型枚举
#[derive(Debug)]
enum FileSystemError {
    FileNotFound { filename: String },
    PermissionDenied { operation: String },
}
