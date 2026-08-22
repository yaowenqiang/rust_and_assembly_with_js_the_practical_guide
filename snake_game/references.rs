fn main() {
    let message = String::from("Hello");
    let message_2 = &message;
    println!("{}", message);
    println!("{}", message_2);
}
