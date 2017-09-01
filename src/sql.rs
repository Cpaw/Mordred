extern crate iron;
extern crate rustc_serialize;

extern crate router;

use iron::prelude::*;
use router::Router;

extern crate postgres;
use postgres::{Connection, TlsMode};


pub struct Userdata {
    pub username: String,
    pub password: String,
    pub score: i16,
}

pub struct Question{
    pub id: i16,
    pub name: String,
    pub statement: String,
    pub accuracy: i8,
}



pub fn database_init(conn: &postgres::Connection){
    //create table (userdata)
}

pub fn create_hoge_table(conn:  &postgres::Connection) {
    // Create Table
    conn.execute("CREATE TABLE userdata(
      username varchar,
      password varchar,
      score int8 NOT NULL DEFAULT 0
      )",
    &[]).unwrap();

}




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
   }*/
