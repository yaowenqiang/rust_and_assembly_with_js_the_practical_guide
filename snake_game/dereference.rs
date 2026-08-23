fn main() {
    let mut message = String::from("Hello");
    // let message_2 = &mut message;
    let message_2 = &message;
    // let mut message_2 = &mut message;
    // message_2.push_str(" world");
    let message_3 = &message_2;
    // (*message_2).push_str(" world");
    println!("{}", message);
    println!("{}", message_3);

    let a = 10;
    let b = &a;
    let mut c = &b;
    let d = b;
    println!("{}", b);
    println!("{}", a == *b);
    println!("{}", a == **c);

    // **c = 100;

    println!("value of a: {}", a);
    println!("address of a: {:p}", &a);
    println!("value of b: {:p}", b);
    println!("value of c: {:p}", c);
    println!("value of d: {:p}", d);

    let e = &&100;
    c = e;
    println!("value of c: {:p}", c);
    println!("value of b: {:p}", e);
    println!("value of c: {:p}", *c);
    println!("value of b: {:p}", *e);
    println!("value of e: {}", **e);
    println!("address of 100: {:p}", &(**e));
}
