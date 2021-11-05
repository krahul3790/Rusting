fn main() {

  let s = String::from("hello");

  takes_ownership(s);
  //println!("{}", s);

  let x = 5;
  makes_copy(x);
  println!("{}", x);

  let mut s1 = String::from("Hello");
  let r1 = &s1;
  let r2 = &s1;

  println!("{}, {}", r1, r2);
  let r3 = &mut s1;
  println!("{}", r1);

}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}
