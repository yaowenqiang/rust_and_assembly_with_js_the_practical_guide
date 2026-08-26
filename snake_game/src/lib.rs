mod another_lib;
use another_lib::another_mod::another_func;

// 新增：模块系统演示
pub mod module_demo;

fn outsider() {
    println!("Outsider!");
    another_func();
}

pub mod learning_rust {
    use std::fmt;
    pub mod top_level {
        pub fn hi_there() {
            println!("Hi, there!");
            crate::outsider();
            super::super::outsider();
        }
        pub mod low_level {
            pub fn hello_world() {
                println!("Hello world!");
            }
        }
    }
    pub trait Log {}

    #[derive(Debug)]
    pub struct Person {
        pub name: String,
        pub last_name: String,
        pub age: u32,
    }

    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "my name is :{} last name is : {}",
                self.name, self.last_name
            )
        }
    }

    impl Person {
        fn new(name: String, last_name: String, age: u32) -> Self {
            Self {
                name,
                last_name,
                age,
            }
        }

        fn change_age(&mut self, age: u32) {
            self.age = age;
        }
    }

    #[derive(Debug)]
    pub struct Animal(pub String);

    pub fn log_info() {
        println!("log info 1");
    }

    pub fn log_info2() {
        println!("log info 2");
        crate::learning_rust::top_level::hi_there();
    }
}
