use std::env;
use std::fs;

fn main() {
  // --snip--
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  println!("In File {}", filename);

  let contents = fs::read_to_string(filename)
      .expect("Something went wrong reading the file");

  println!("With text:\n{}", contents);
}
