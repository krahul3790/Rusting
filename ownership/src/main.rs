fn main() {

  // Several threads can read data. 
  // Only one thread can write/modify data. 
  // State / Data / Values. 
  let value = 1000;
  let ref = &value;

  println!("{}, {}",value, ref);


  // Ownership Rules 
  // Every value in Rust has a variable that's called its owner. 
  // There can only be 1 owner at a time for a value. 
  // When the owner goes out of scope, then the value will be dropped. 


  // Question is when to borrow and when to copy. 
  // Please borrow this and become the owner, iam dead. 

  let me = String::from("Hello");
  // me is the owner of the data Hello/value in memory. 
  // No other owner for this value. 
  //
  let other = me;
  // Now other is the owner of data pointed to by me. 

  // The values is borrowed by the other owner. 
  // Owner can lend the values to others. 
  //
  // Any type which implementes the copy trait copies the value to the new owner

  let mut a = 1;
  let b = String::from("ABCDF");
  // b is the owner of the ABCDF data / memory / state. 
  let ref = &mut a;
  println!("{}", ref);
  println!("{}", other);
}

