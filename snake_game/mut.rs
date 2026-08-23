fn main() {
    let mut message = String::from("Hello");
    let message_2: &mut String = &mut message;
    message_2.push_str(" world");
    println!("{}", message);
    // println!("{}", message);
    println!("{}", message_2);

    // let message3 = String::from("Hello");
    // let mut message4 = message3;
    // message4.push_str(" world");
    // // println!("{}", message);
    // println!("{}", message4);
}
