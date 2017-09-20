#[macro_use] extern crate iron;
#[macro_use] extern crate router;
extern crate rustc_serialize;
extern crate urlencoded;
extern crate iron_sessionstorage;
extern crate mime;
extern crate csv;
extern crate bodyparser;
extern crate persistent;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::io::Read;
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
mod csv_parser;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use std::collections::HashMap;

extern crate params;
extern crate handlebars_iron as hbs;
extern crate handlebars;
use handlebars::Handlebars;
use std::io::prelude::*;
use iron::{headers, status};
use std::path::Path;
use iron::modifiers::{Redirect,Header};

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
    let (username, password) = {
        let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
        (iexpect!(formdata.get("username"))[0].to_owned(), iexpect!(formdata.get("password"))[0].to_owned())
    };

    // postgreのコネクション作成
    let dsn = "postgres://dev:secret@localhost";
    let conn = Connection::connect(dsn, TlsMode::None).unwrap();

    let mut status: bool = false;
    if is_user_exists(&conn, username.to_string(), password.to_string()){
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
    // postgreのコネクション作成
    let dsn = "postgres://dev:secret@localhost";
    let conn = Connection::connect(dsn, TlsMode::None).unwrap();;

    let problems = show_problems(&conn);
    println!("{:?}", problems);

    let mut res = HashMap::new();
    for problem in problems {
        res.insert(problem.0, problem.1);
    }
    //let response = serde_json::to_string(&res);

    Ok(Response::with((
        status::Ok,
        format!("{:?}", res)
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

//問題追加
fn add_problem(req: &mut Request) -> IronResult<Response> {
    // ここにあとでadminユーザでログインしているかの判定を付け加える

    let (title, description, score, accuracy) = {
        let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
        (iexpect!(formdata.get("title"))[0].to_owned(),
         iexpect!(formdata.get("description"))[0].to_owned(),
         iexpect!(formdata.get("score"))[0].to_owned(),
         iexpect!(formdata.get("accuracy"))[0].to_owned()
        )
    };

    // postgreのコネクション作成
    let dsn = "postgres://dev:secret@localhost";
    let conn = Connection::connect(dsn, TlsMode::None).unwrap();;

    insert_problem(&conn, title.to_string(), description.to_string(), score.parse::<i16>().unwrap(), accuracy.parse::<f64>().unwrap());
    let status: bool = true;

    return Ok(Response::with(
        (status::Ok,
         format!("{{\"status\": {}}}", status)
        )))
}

fn user(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        format!("test")
    )))
}



fn upload(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};
    println!("[+] POST /upload");

    //bodyparserじゃmultipart/form-dataのデータ受け取れない
    //println!("{:?}", req.get::<bodyparser::Raw>());

    //println!("Params = {:?}",req.get_ref::<Params>().unwrap());


    let map = req.get_ref::<Params>().unwrap();

    //println!("file = {:?}",map.find(&["file"]));

    match map.find(&["file"]){
        Some(&Value::File(ref file)) => {
            //println!("{:?}",file.path.to_string_lossy());
            //println!("{}",file.path.as_path());

            let path = file.path.as_path();
            let display = path.display();

            // pathを読み込み専用モードで開く。これは`io::Result<File>`を返す。
            let mut file = match File::open(&path) {
                // `io::Error`の`description`メソッドはエラーを説明する文字列を返す。
                Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
                Ok(file) => file,
            };

            // ファイルの中身を文字列に読み込む。`io::Result<useize>`を返す。
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
                Ok(_) => {
                    if s != "" {
                        let splitted = csv_parser::parse(s.as_str()); //Stringだとlifetimeの関係上引数で渡せない
                        println!("splitted = {:?}", splitted);
                        println!("{}",csv_parser::check(splitted, 3));
                    }
                }
            }

        }
        _ => println!(""),
    }
    Ok(Response::with((status::Ok, "Uploaded!")))
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
        add_problem: post "/problem/" => add_problem,
        answer: post "/problem/:id" => answer,

        upload: post "/upload" => upload,
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

    insert_problem(&conn, "問題1".to_string(), "あああああああああああああ".to_string(), 30, 50.356);
    insert_problem(&conn, "問題2".to_string(), "いいいいいいいいいいいいいい".to_string(), 100, 0.045);
    insert_problem(&conn, "problem3".to_string(), "uuuuuuuuuuuuuuu".to_string(), 150, 33.387);

    println!("{}", get_description(&conn, 2));


}
