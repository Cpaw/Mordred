extern crate iron;
extern crate rustc_serialize;

#[macro_use] extern crate router;

use iron::prelude::*;
//use router::Router;

mod hello_world;
mod set_greeting;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {
//    let mut router = Router::new();

    let router = router!(
        hello_world: get"/" => hello_world::hello_world,
        greet: post"/set" => set_greeting::set_greeting,
    );

//    router.get("/", hello_world::hello_world, "hello_world");
//    router.post("/set", set_greeting::set_greeting, "set_greeting");


    Iron::new(router).http("localhost:3000").unwrap();
    println!("on 3000");
}
