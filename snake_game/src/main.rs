use snake_game::learning_rust::{Animal, Person, log_info, log_info2};
use snake_game::learning_rust::{top_level::hi_there, top_level::low_level::hello_world};

fn main() {
    let person = Person {
        name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
    };
    println!("{}", person);
    println!("{:?}", person);
    let animal = Animal(String::from("Dog"));
    println!("{:?}", animal);

    log_info();
    log_info2();
    hello_world();
    hi_there();
    println!("{}", person.name);
}
