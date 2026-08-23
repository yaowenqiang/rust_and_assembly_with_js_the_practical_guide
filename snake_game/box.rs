#[derive(Debug)]
struct Person {
    name: String,
    last_name: String,
    age: u32,
}

impl Person {
    fn some_function() {
        println!("Some_function");
    }
    fn from(name: String, last_name: String, age: u32) -> Self {
        Self {
            name,
            last_name,
            age,
        }
    }

    fn new(name: &str, last_name: &str, age: u32) -> Self {
        // Self {
        //     name: String::from(name),
        //     last_name: String::from(last_name),
        //     age,
        // }
        Self::from(String::from(name), String::from(last_name), age)
    }

    fn change_age(&mut self, age: u32) {
        println!("Current age: {}", self.age);
        self.age = age;
        println!("Current age: {}", self.age);
    }
}

fn main() {
    let num = 32;
    let number_1 = Box::new(100);
    println!("{}", number_1);

    let person = Person {
        name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
    };

    let mut person2 = Person::new("name", "last_name", 10);
    println!("{:#?}", person);
    println!("{:#?}", person2);
    Person::some_function();

    person2.change_age(20);
    println!("{:#?}", person2);
}
