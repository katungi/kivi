fn main() {
    // get all arguments sent by user
  let mut arguments = std::env::args().skip(1);

  // get the next value in the arguments --> we get back an option string because the option might be there or not String or null
  // sigh, hakuna null kwa Rust

  let key = arguments.next().unwrap();
  let value = arguments.next().unwrap();
  // println!("The Key is '{}' and the value is {}", key, value);


}
