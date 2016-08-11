extern crate Twitter4Rust;

use Twitter4Rust::{Keys, Parameter, Method, request};
use std::collections::LinkedList;

fn main() {
  let keys = Keys::new(
    "Your Consumer Key",
    "Your Consuemr Secret",
    "Your Access Token",
    "Your Access Token Secret"
  );

  let params: LinkedList<Parameter> = LinkedList::new();
  println!("{}", request(&keys, Method::Get, "account/verify_credentials.json", params));

  let mut params2: LinkedList<Parameter> = LinkedList::new();

  params2.push_back(Parameter::new("status", "Hello World From Rust!!!"));

  println!("result -> {}", request(&keys, Method::Post, "/statuses/update.json", params2));
}

