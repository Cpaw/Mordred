extern crate iron;
extern crate rustc_serialize;

extern crate router;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use std::io::Read;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Greeting {
    msg: String
}

pub fn set_greeting(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request: Greeting = json::decode(&payload).unwrap();
    let greeting = Greeting { msg: request.msg };
    let payload = json::encode(&greeting).unwrap();
    println!("{}", payload);
    Ok(Response::with((status::Ok, payload)))

}
