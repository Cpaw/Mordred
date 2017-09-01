#[macro_use] extern crate iron;
#[macro_use] extern crate router;
extern crate rustc_serialize;
extern crate urlencoded;
extern crate iron_sessionstorage;
extern crate mime;
extern crate csv;
extern crate serde_json;
extern crate bodyparser;
extern crate persistent;
#[macro_use]
extern crate serde_derive;

use std::io::Read;
use iron::status;
use iron::modifiers::Redirect;
use router::{Router, url_for};
use rustc_serialize::json;
use iron_sessionstorage::traits::*;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;
use iron::headers::ContentType;
use iron::prelude::*;
use serde_json::Value;

use urlencoded::UrlEncodedBody;

extern crate postgres;
use postgres::{Connection, TlsMode};

mod sql;
use sql::*;
mod hello_world;
mod set_greeting;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

struct Login {
    username: String
}


struct Problem {
    p1: u16,
    p2: u16,
    p3: u16,
    p4: u16,
    p5: u16,
}

// セッション情報
impl iron_sessionstorage::Value for Login {
    fn get_key() -> &'static str { "logged_in_user" }
    fn into_raw(self) -> String { self.username }
    fn from_raw(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            Some(Login { username: value })
        }
    }
}

// ログイン処理
fn login(req: &mut Request) -> IronResult<Response> {
    let mut status: bool = false;
    if try!(req.session().get::<Login>()).is_some() {
        status = true;
    }
    Ok(Response::with((
        ContentType::json().0,
        status::Ok,
        format!("{{\"status\": {}}}", status)
    )))
}

// ユーザログイン
fn login_post(req: &mut Request) -> IronResult<Response> {
    let username = {
        let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
        iexpect!(formdata.get("username"))[0].to_owned()
    };

    // postgreのコネクション作成
    let dsn = "postgres://dev:secret@localhost";
    let conn = Connection::connect(dsn, TlsMode::None).unwrap();;

    let mut status: bool = false;
    if is_user_exists(&conn, username.to_string()){
        try!(req.session().set(Login { username: username }));
        status = true;
    }

    Ok(Response::with((
        status::Ok,
        format!("{{\"status\": {}}}", status)
    )))
}

// ログアウト
fn logout(req: &mut Request) -> IronResult<Response> {
    try!(req.session().clear());
    let mut status: bool = true;
    Ok(Response::with((
        status::Ok,
        format!("{{\"status\": {}}}", status)
    )))
}

// ユーザ登録
fn register(req: &mut Request) -> IronResult<Response> {
    let (username, password) = {
        let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
        (iexpect!(formdata.get("username"))[0].to_owned(), iexpect!(formdata.get("password"))[0].to_owned())
    };

    // postgreのコネクション作成
    let dsn = "postgres://dev:secret@localhost";
    let conn = Connection::connect(dsn, TlsMode::None).unwrap();;

    insert_userdata(&conn, username, password);

    Ok(Response::with((
        status::Ok,
        format!("test")
    )))
}

// 問題一覧
fn problems(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        format!("test")
    )))
}

// 問題詳細
fn problem(req: &mut Request) -> IronResult<Response> {
    let ref router = req.extensions.get::<Router>();
    let ref problem_id = router
        .unwrap()
        .find("id")
        .unwrap();

    return Ok(Response::with(
        (status::Ok,
         format!("Hello {}", problem_id).as_str()
        )))
}

// 問題回答
fn answer(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    {
        let ref router = &req.extensions.get::<Router>();
        let ref problem_id = router
            .unwrap()
            .find("id")
            .unwrap();

        //受け取ったデータをstringでpayloadに格納
        req.body.read_to_string(&mut payload);
    }

    let mut json_body = &mut req.get::<bodyparser::Json>();
    println!("{:?}", json_body);

    Ok(Response::with((
        status::Ok,
        format!("test")
    )))
}

fn user(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        format!("test")
    )))
}

fn main() {
    // ルーティング作成
    let router = router!(
        login: get "/login" => login,
        login_post: post "/login" => login_post,
        logout: get "/logout" => logout,
        regist: post "/regist" => register,
        user: get "/user" => user,
        problems: get "/problems" => problems,
        problem: get "/problem/:id" => problem,
        answer: post "/problem/:id" => answer,
    );

    let my_secret = b"verysecret".to_vec();
    let mut ch = Chain::new(router);
    ch.link_around(SessionStorage::new(SignedCookieBackend::new(my_secret)));
    let _res = Iron::new(ch).http("0.0.0.0:3000").unwrap();
    println!("on 3000");


    //PostgreSQL
    let dsn = "postgres://dev:secret@localhost";
    let conn = match Connection::connect(dsn, TlsMode::None) {
        Ok(conn) => conn,
        Err(e) => {
               println!("Connection error: {}", e);
               return;
        }
     };

    database_init(&conn);

    insert_userdata(&conn, "金田".to_string(), "gomigomi".to_string());
    insert_userdata(&conn, "山田".to_string(), "nemiiiiiiii".to_string());
    insert_userdata(&conn, "吉岡".to_string(), "1234567890".to_string());




    insert_question(&conn, "くそ2".to_string(), "あああああああああああああ".to_string(), 30, 50.356);


    let res = is_user_exists(&conn, "山田".to_string());

    if res == true{
        println!("登録済み");
    }else{
        println!("いないよ");
    }


    let id: i32 = 1;
    let username = "山田".to_string();
    add_score(&conn, id, username);
}
