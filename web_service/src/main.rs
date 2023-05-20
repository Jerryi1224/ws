mod models;


fn main() {
    println!("Hello, world!");
    let mut a = 1;
    let b = &mut a;
    *b = 2;
    println!("{}", a);
}
