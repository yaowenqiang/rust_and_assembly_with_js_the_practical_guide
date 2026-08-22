fn main() {
    let mut message = String::from("Hello");
    // print_message(message);
    // extend_message(message);
    message = extend_message(message);
    extend_message(message);
    let b = 10;
    extend_age(b);
    println!("{}", b);
}

fn print_message(a: String) {
    println!("{}", a);
    let c = a;
}

fn extend_message(mut a: String) -> String {
    a.push_str(" worlcd");
    a
}

fn extend_age(mut a: u32) {
    a += 100;
}
