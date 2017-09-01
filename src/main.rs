extern crate iron;
extern crate rustc_serialize;

extern crate router;

use iron::prelude::*;
use router::Router;

mod hello_world;
mod set_greeting;


//mod sql;
/*
#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}
*/
extern crate postgres;
use postgres::{Connection, TlsMode};

/*struct Counter {
    id: i32,
    counter: i16,
}*/

mod sql;
use sql::*;

fn main() {
    /*
    let mut router = Router::new();

    router.get("/", hello_world::hello_world, "hello_world");
    router.post("/set", set_greeting::set_greeting, "set_greeting");
    */


    /*
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting {
            msg: "Hello, world".to_string()
        };
        let payload = json::encode(&greeting).unwrap();
        println!("{}", payload);
        Ok(Response::with((status::Ok, payload)))
    }

    fn set_greeting(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let request: Greeting = json::decode(&payload).unwrap();
        let greeting = Greeting { msg: request.msg };
        let payload = json::encode(&greeting).unwrap();
        println!("{}", payload);
        Ok(Response::with((status::Ok, payload)))

    }
    */

    /*
    Iron::new(router).http("localhost:3000").unwrap();
    println!("on 3000");
    */

    let dsn = "postgres://dev:secret@localhost";
       let conn = match Connection::connect(dsn, TlsMode::None) {
           Ok(conn) => conn,
           Err(e) => {
               println!("Connection error: {}", e);
               return;
           }
       };

       //create_schema(&conn);
       create_hoge_table(&conn);
/*
       // Create Table
       conn.execute("CREATE TABLE hoge (
         id SERIAL,
         hoge SMALLINT NOT NULL DEFAULT 0
         )",
       &[]).unwrap();*/

/*
       // Counter type
       let me = Counter {
           id: 0,
           counter: 1
       };

       // Interting
       let stmt = match conn.prepare("INSERT INTO hoge (hoge) VALUES ($1)") {
           Ok(stmt) => stmt,
           Err(e) => {
               println!("Preparing query failed: {}", e);
               return;
           }
       };

       // Run
       stmt.execute(&[&me.counter]).ok().expect("Inserting counter failed");

       for row in &conn.query("SELECT id, hoge FROM hoge", &[]).unwrap() {
           let counter = Counter {
               id: row.get(0),
               counter: row.get(1)
           };
           println!("Found hoge {}", counter.counter);
       }
*/







}
