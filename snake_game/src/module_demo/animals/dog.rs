pub struct Dog {
    pub name: String,
    age: u32,
}

impl Dog {
    pub fn new(name: String, age: u32) -> Self {
        Dog { name, age }
    }

    pub fn bark(&self) {
        println!("🐕 {} ({}岁) 说: 汪汪！", self.name, self.age);
    }

    pub fn sleep(&self) {
        println!("🐕 {} 在睡觉...", self.name);
    }
}

pub fn create_dog(name: &str, age: u32) -> Dog {
    Dog {
        name: name.to_string(),
        age,
    }
}
