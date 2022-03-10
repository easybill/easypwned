use crate::bloom_create::bloom_create;

pub mod bloom_create;

fn main() {
    match bloom_create() {
        Ok(()) => {},
        Err(e) => println!("could not create bloom: {}", e),
    };

    println!("Hello, world!");
}
