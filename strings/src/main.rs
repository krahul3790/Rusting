fn main() {

  let data = "initial contents";
  let s = data.to_string();

  let s = "initial_contents".to_string();

  let s = String::from("initial contents");

  let mut s1 = String::from("Hello");
  let s2 = "data";
  s1.push_str(s2);
  println!("{} - {}", s1, s2);

  let s3 = s1 + &s2;
  println!("{}", s3);

  // Each character in the string is 1 byte and encoded in UTF-8.
  let s4 = String::from("Helloa");

  println!("{}", &s4[0..100]);
}
