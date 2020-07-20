

pub fn ownership_test() {
  let mut s = String::from("hello from ownership");  // s comes into scope

  s = takes_ownership(s);             // s's value moves into the function and so is no longer valid here
  println!("{}", s);

  let x = 5;                      // x comes into scope

  makes_copy(x);                  // x would move into the function,
                                  // but i32 is Copy, so it’s okay to still
                                  // use x afterward
  s = String::from("Now this is new string");
  println!("{}", s);
  println!("This int ({}) implements the Copy trait, so the function does not lose ownership", x);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.


fn takes_ownership(mut some_string: String) -> String { // some_string comes into scope
  some_string += "Currenly I [fn takes_ownership] own this string";
  some_string
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.