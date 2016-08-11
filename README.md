# Twitter4Rust

## About this
The Simple Twitter API Wrapper Library For Rust.  
  
  
## Features

- REST API(POST/GET)

Currently, this library doesn't support streaming api.  
  
This library privides primitive `request` function only.(but I'll develop the another function to handle streaming api.)  
Twitter provides too many APIs, but most of them aren't likely to be used. It is difficult that implement functions for all API and I don't have time to manage such a function. You can fork this library and you can implement the function for each of endpoints if you need.  
  
  
## Sample
You can access twitter api with simple way.  
This library is designed like my Twitter library [Twitter4D](https://github.com/alphaKAI/Twitter4D) and [T4C](https://github.com/alphaKAI/t4c).  
  
  
### This Sample Requires

```rust
extern crate Twitter4Rust;
use Twitter4Rust::{Keys, Parameter, Method, request};
use std::collections::LinkedList;
```

### Create Keys

```rust
let keys = Keys::new(
    "Your Consumer Key",
    "Your Consumer Secret",
    "Your Access Token",
    "Your Access Token Secret");
```

### POST API SAMPLE : statuses/update.json 

```rust
  let mut params: LinkedList<Parameter> = LinkedList::new();

  params.push_back(Parameter::new("status", "Hello World From Rust!!!"));

  println!("result -> {}", request(&keys, Method::Post, "/statuses/update.json", params));
```


### GET API SAMPLE : account/verify_credentials.json

```rust
  let params: LinkedList<Parameter> = LinkedList::new();
  println!("result -> {}", request(&keys, Method::Get, "account/verify_credentials.json", params));

```
***Note*** : Current request function requries parameters as `LinkedList<Parameter>`, but this style isn't smart, I'll replace it by HashMap.  
  
  
## LICENSE
This library is relased under the MIT License, see LICENSE file for the details.  
Copyright (C) 2016 alphaKAI  
