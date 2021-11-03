use std::env;

fn main() {
    println!("Hello, world!");

    let args:Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
}
