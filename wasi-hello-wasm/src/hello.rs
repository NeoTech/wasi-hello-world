fn main() {
    println!("Hello, world!");
    println!("{}", add(5, 8));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}