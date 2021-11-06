fn main() {

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

  println!("{}", other);
}

