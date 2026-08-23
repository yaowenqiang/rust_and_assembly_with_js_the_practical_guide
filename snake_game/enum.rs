#[derive(Debug)]
enum PersonId {
    Passport(String),
    // Passport(u32),
    IdentityCard(String),
    // IdentityCard(u32, u32,u32),
}

#[derive(Debug)]
struct Person {
    name: String,
    last_name: String,
    age: u32,
    id: PersonId,
}

#[derive(Debug)]
struct Animal(String, u32, String);

impl Person {
    fn new() -> Self {
        Self {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
            id: PersonId::Passport("aHsfjsdjsljdls".to_string()),
        }
    }

    fn from(name: String, last_name: String, age: u32, id: PersonId) -> Self {
        Self {
            name,
            last_name,
            age,
            id,
        }
    }

    fn display_info(&self) {
        println!(
            "person: {} {} {} {:#?}",
            self.name, self.last_name, self.age, self.id
        )
    }
}

fn main() {
    let person = Person::from(
        "John".to_string(),
        "Doe".to_string(),
        30,
        PersonId::Passport("AHasdkfjsfds".to_string()),
    );

    println!("{:?}", person);
    person.display_info();
    check_person_id(&person.id);

    let animal = Animal("Dog".to_string(), 5, "Brown".to_string());
    let Animal(animal_name, _, _) = Animal("Dog".to_string(), 5, "Brown".to_string());
    println!("{:?}", animal);
    println!("{:?}", animal_name);
}

fn check_person_id(id: &PersonId) {
    if let PersonId::Passport(_) = id {
        println!("Passport");
    } else {
        println!("IdentityCard");
    }

    match id {
        PersonId::Passport(_) => println!("Passport"),
        PersonId::IdentityCard(_) => println!("IdentityCard"),
    }
}
