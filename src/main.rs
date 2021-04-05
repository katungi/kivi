use std::fs;

fn main() {
    // get all arguments sent by user
  let mut arguments = std::env::args().skip(1);

  // get the next value in the arguments --> we get back an option string because the option might be there or not String or null
  // sigh, hakuna null kwa Rust

  let key = arguments.next().expect("Key was not found");
  let value = arguments.next().unwrap();
  // println!("The Key is '{}' and the value is {}", key, value);
  // we need a place to save the data. For now we will use a file.

  let contents = format!("{}\t{}\n",key, value);
  fs::write("kv.db", contents);

}
