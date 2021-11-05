fn main() {
  let x = 5; // Immtuable but can be shadowed. 
  println!("{}", x);
  let x = x + 1; // Shadowed
  println!("{}", x);
  {
    let x = 1009;
    println!("{}", x);
  }
  println!("{}", x);
  println!("Hello, world!");
}
