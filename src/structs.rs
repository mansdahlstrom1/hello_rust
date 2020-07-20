struct User {
  email: String,
  first_name: String,
  last_name: String,
}

pub fn test_users() {
  let user1 = User {
    email: String::from("mans@mansdahlstrom.se"),
    first_name: String::from("Måns"),
    last_name: String::from("Dahlström"),
  };
  
  let user2 = User {
    last_name: String::from("Dahlan"),
    ..user1
  };
  
  let user3 = User {
    last_name: String::from("Dahlan"),
    ..user2
  };


  println!("are they all the same? {}, {} {}", user1.last_name, user2.last_name, user3.last_name);
}


