struct User {
  active: bool,
  username: String, 
  email: String,
  sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
  User {
    email: email,
    username: username,
    active: true,
    sign_in_count:1,
  }
}


fn main() {

  let user1 = User {
    email: String::from("someusername1232"),
    username: String::from("someusername"),
    active: true,
    sign_in_count: 1,
  };

  println!("{}", user1.email);

  let mut user2 = User {
    email: String::from("hello123232"),
    username: String::from("hello"),
    active: false,
    sign_in_count: 20,
  };
  
  user2.sign_in_count = 2434;

  let username = String::from("Rahul");
  let usernameAlias = String::from("Rahul@123");

  println!("{}", build_user(username, usernameAlias).username);    
  println!("{}", user2.sign_in_count);
  println!("Hello, world!");

  let user3 = User {
    email: String::from("213123"),
    ..user2
  };
  
  println!("{}", user3.username);

}
