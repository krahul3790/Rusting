fn main() {
  let s = String::from("Hello");
  let length = compute_length(&s);
  // What heppns here is that &s creates a reference
  // that refers to the value of s but does not own it. 
  // Because it does not own it, the value it points to will
  // not be dropped when the reference is stopped being used. 

  println!("{}, {}",s, length);
  println!("Hello, world!");

  change(&s);
  println!("{}", s);
}

fn change(string: &String) {
  string.push_str(", world");
}

fn compute_length(s: &String) -> usize {
  s.len()
}
