extern crate iron;
extern crate rustc_serialize;

extern crate router;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Greeting {
    msg: String
}

pub fn hello_world(_: &mut Request) -> IronResult<Response> {
    let greeting = Greeting {
        msg: "Hello, world".to_string()
    };
    let payload = json::encode(&greeting).unwrap();
    println!("{}", payload);
    Ok(Response::with((status::Ok, payload)))
}
