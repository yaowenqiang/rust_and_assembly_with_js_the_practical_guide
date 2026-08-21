fn main() {
    let mut message = "Hello, world!";
    println!("{}", message);
    message = "Hello, Rust!";
    println!("{}", message);
    let returned_message = print_welcome(message);
    println!("{}", returned_message);
    println!("{}", message);
    print_integer_numbers();
    print_float_numbers();
}

fn print_welcome(message: &str) -> &str {
    println!("{}", message);
    "Hi there back"
}

fn print_integer_numbers() {
    let custom_num = 90_000;
    let hex_num = 0xfa;
    let bin_num = 0b0010_1011;
    let byte_num = b'A';
    println!("{} {} {} {}", custom_num, hex_num, bin_num, byte_num);
}

fn print_float_numbers() {
    let float_num = 3.14;
    let float_num_2 = 3.2;
    let float_num_3 = 0.1_f32;
    let tup: (i32, &str, u8) = (20, "hello", 1);
    println!(
        "{} {} {} {} {} {}",
        float_num, float_num_2, float_num_3, tup.0, tup.1, tup.2
    );
    println!("{:?}", tup);

    let (a, b, c) = tup;
    println!("{} {} {}", a, b, c);

    let x: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} {} {} {} {}", x[0], x[1], x[2], x[3], x[4]);
    let i = [2; 6];
    println!("{:?}", i);
}
