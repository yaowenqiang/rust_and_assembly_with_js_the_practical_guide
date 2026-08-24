pub struct Cat {
    pub name: String,
    pub color: String,
}

impl Cat {
    pub fn new(name: String, color: String) -> Self {
        Cat { name, color }
    }

    pub fn meow(&self) {
        println!("🐱 {} ({}) 说: 喵喵！", self.name, self.color);
    }

    pub fn scratch(&self) {
        println!("🐱 {} 在抓挠...", self.name);
    }
}

impl Default for Cat {
    fn default() -> Self {
        Cat {
            name: "未命名猫".to_string(),
            color: "未知颜色".to_string(),
        }
    }
}
