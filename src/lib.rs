extern crate rustc_serialize;
extern crate crypto;
extern crate curl;
extern crate time;

use crypto::hmac::Hmac;
use crypto::sha1::Sha1;
use crypto::mac::Mac;
use rustc_serialize::base64::{ToBase64, STANDARD};
use std::collections::LinkedList;
use curl::easy::{Easy, List};
use std::io::Read;

fn url_encode(s: &str) -> String {
  let mut en = String::new();

  for k in 0..s.len() {

    if url_unreserved(s.chars().nth(k).unwrap()) {
      en.push_str(&s.chars().nth(k).unwrap().to_string());
    } else {
      let ss = s.chars().nth(k).unwrap() as i32;
      en.push_str(&format!("%{:02X}", ss));
    }
  }

  return en;
}

fn url_unreserved(c: char) -> bool {
  if 'a' <= c && c <= 'z' {
    return true;
  }
  if '0' <= c && c <= '9' {
    return true;
  }
  if 'A' <= c && c <= 'Z' {
    return true;
  }
  if c == '-' || c == '.' || c == '_' || c == '~' {
    return true;
  }
  return false;
}

static BASE_URL: &'static str = "https://api.twitter.com/1.1/";

#[derive(Copy, Clone)]
pub enum Method {
  Post,
  Get
}

#[derive(Clone)]
pub struct Parameter {
  key: String,
  value: String
}

impl Parameter {
  pub fn new(k: &str, v: &str) -> Parameter {
    Parameter {
      key: k.to_string(),
      value: v.to_string()
    }
  }
}

fn join_parameters(params: &LinkedList<Parameter>, sep: &str) -> String {
  let joined: Vec<_> =
    params.into_iter().map(|param| format!("{0}={1}", param.key, param.value)).collect();

  return joined.join(sep);
}

pub struct Keys {
  consumer_key: String,
  consumer_secret: String,
  access_token: String,
  access_token_secret: String
}

impl Keys {
  pub fn new(consumer_key: &str,
             consumer_secret: &str,
             access_token: &str,
             access_token_secret: &str)
             -> Keys {
    Keys {
      consumer_key: consumer_key.to_string(),
      consumer_secret: consumer_secret.to_string(),
      access_token: access_token.to_string(),
      access_token_secret: access_token_secret.to_string()
    }
  }
}

fn signature(consumer_secret: &str,
             access_token_secret: &str,
             method: Method,
             url: &str,
             params: &LinkedList<Parameter>)
             -> String {
  let query = join_parameters(params, "&");

  let consumer_secret = url_encode(consumer_secret);
  let access_token_secret = url_encode(access_token_secret);
  let key = format!("{}&{}", consumer_secret, access_token_secret);

  let method = match method {
    Method::Post => "POST",
    Method::Get => "GET",
  };


  let msg: Vec<String> = vec![method, &url, &query].into_iter().map(|x| url_encode(x)).collect();
  let msg = msg.join("&");

  let mut res = Hmac::new(Sha1::new(), &key.into_bytes());
  res.input(&msg.into_bytes());

  let ret = res.result().code().to_base64(STANDARD);


  String::from(url_encode(&ret))
}

fn gen_oauth_params(keys: &Keys) -> LinkedList<Parameter> {
  let now_unixtime: i64 = time::now().to_timespec().sec;

  let mut params: LinkedList<Parameter> = LinkedList::new();
  params.push_back(Parameter::new("oauth_consumer_key", &keys.consumer_key));
  params.push_back(Parameter::new("oauth_nonce", &now_unixtime.to_string()));
  params.push_back(Parameter::new("oauth_signature_method", "HMAC-SHA1"));
  params.push_back(Parameter::new("oauth_timestamp", &now_unixtime.to_string()));
  params.push_back(Parameter::new("oauth_token", &keys.access_token));
  params.push_back(Parameter::new("oauth_version", "1.0"));

  params
}

fn build_params(oauth_params: LinkedList<Parameter>,
                additional_params: LinkedList<Parameter>)
                -> LinkedList<Parameter> {
  let mut params: LinkedList<Parameter> = LinkedList::new();

  for e in oauth_params {
    params.push_back(e);
  }

  for e in additional_params {
    let k = Parameter {
      key: url_encode(&e.key),
      value: url_encode(&e.value)
    };

    params.push_back(k);
  }

  params
}

pub fn request(keys: &Keys,
               method: Method,
               end_point: &str,
               argument_params: LinkedList<Parameter>)
               -> String {

  let mut oauth_params: LinkedList<Parameter> = gen_oauth_params(keys);
  let mut params: LinkedList<Parameter> = build_params(oauth_params.clone(),
                                                       argument_params.clone());

  let url = format!("{}{}", BASE_URL, end_point);

  let oauth_signature = signature(&keys.consumer_secret,
                                  &keys.access_token_secret,
                                  method,
                                  &url,
                                  &params);

  oauth_params.push_back(Parameter::new("oauth_signature", &oauth_signature));
  params.push_back(Parameter::new("oauth_signature", &oauth_signature));

  let authorize = format!("Authorization: OAuth {}",
                          join_parameters(&oauth_params, ","));

  let path = join_parameters(&params, "&");

  let mut dst = Vec::new();
  let mut easy = Easy::new();

  let mut list = List::new();
  list.append(&authorize).unwrap();
  easy.http_headers(list).unwrap();

  match method {
    Method::Get => {
      easy.url(&format!("{}?{}", url, path)).unwrap();

      let mut transfer = easy.transfer();

      transfer.write_function(|data| {
          dst.extend_from_slice(data);
          Ok(data.len())
        })
        .unwrap();

      transfer.perform().unwrap();
    }
    Method::Post => {
      easy.post(true).unwrap();
      easy.post_field_size(path.len() as u64).unwrap();
      easy.url(&url).unwrap();

      let mut transfer = easy.transfer();

      transfer.read_function(|buf| Ok(path.as_bytes().read(buf).unwrap_or(0)))
        .unwrap();

      transfer.write_function(|data| {
          dst.extend_from_slice(data);
          Ok(data.len())
        })
        .unwrap();

      transfer.perform().unwrap();
    }
  }


  let ret = String::from_utf8(dst).expect("failed to convert u8(dst) to String(ret)");

  ret
}
