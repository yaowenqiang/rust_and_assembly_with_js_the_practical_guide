trait Log {
    fn display_info(&self) -> String;
    fn alert_something(&self) {
        println!("Default implementation called");
    }
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    last_name: String,
    age: u32,
}
struct Animal(String);
struct City(String);

impl Log for Animal {
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
}

fn main() {
    let person = Person {
        name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
    };
    let person_2 = person.clone();
    println!("{}", person.display_info());

    let animal = Animal(String::from("Dog"));
    println!("{}", animal.display_info());
    animal.alert_something();
    person.alert_something();
    // log_info(animal);
    log_info(person);
    let city = City(String::from("New York"));
    // log_info(city);
    log_info_2(&animal);
    log_info_2(&person_2);
}

fn log_info(value: impl Log) {
    println!("{}", value.display_info());
    value.alert_something();
}

fn log_info_2(value: &dyn Log) {
    println!("{}", value.display_info());
    value.alert_something();
}
