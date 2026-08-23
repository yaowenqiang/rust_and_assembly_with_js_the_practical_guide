fn main() {
    let mut message = String::from("Hello");
    //let slice = &message[2..4];
    let slice = &message[2..=4];
    let slice = &message[..];
    println!("{}", slice);

    let mut message2 = String::from("World");
    message2.clear();
    println!("{}", message2);
    move_me(message2);

    let message3 = message.clone();
    println!("{}", message3);
}

fn move_me(val: String) {}
