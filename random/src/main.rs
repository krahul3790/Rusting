use rand::Rng;
use std::collections::HashMap;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Hello, world - {}", secret_number);

    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("key = 1, value = {}", map.get(1));
}
