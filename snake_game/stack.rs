fn main() {
    let a = 10;
    let b = a;
    let c = 15;
    let d = add(a,b);
    println!("{}",d);
}
fn add(a: u32, b: u32) -> u32 {
    let sum = a + b;
    sum
}
// xxd -g1 stack
