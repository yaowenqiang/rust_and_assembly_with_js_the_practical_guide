use wasm_bindgen::prelude::*;

// 当 WebAssembly 模块完成加载时调用
#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"🚀 Hello from WebAssembly!".into());
}

// 导出函数供 JavaScript 调用
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! 欢迎使用 Rust WebAssembly!", name)
}

#[wasm_bindgen]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn calculate_circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

#[wasm_bindgen]
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

#[wasm_bindgen]
pub fn sum_array(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

#[wasm_bindgen]
pub fn log_message(message: &str) {
    web_sys::console::log_1(&message.into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World! 欢迎使用 Rust WebAssembly!");
    }

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(5, 3), 8);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
    }
}
