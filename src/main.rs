extern crate iron;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Client {
    name: String,
    phone: usize,
    delivery_addr: String,
}
