#[macro_use] extern crate iron;
#[macro_use] extern crate router;
extern crate rustc_serialize;
extern crate urlencoded;
extern crate iron_sessionstorage;

use iron::status;
use iron::modifiers::Redirect;
use router::{Router, url_for};

use iron_sessionstorage::traits::*;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

use urlencoded::UrlEncodedBody;
use iron::headers::ContentType;
use iron::prelude::*;

extern crate postgres;
use postgres::{Connection, TlsMode};

mod sql;
use sql::*;
mod hello_world;
mod set_greeting;


struct User {
    name: String
}

// ログイン処理
fn login(req: &mut Request) -> IronResult<Response> {
    let status = "ok".to_string();
    Ok(Response::with((
        ContentType::json().0,
        status::Ok,
        "text/html".parse::<iron::mime::Mime>().unwrap(),
        format!("{{\"status\": {}}}", status)
        // ログインしているかどうか確認する処理を書く
    )))
}

// ユーザログイン                       
fn login_post(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        format!("test")
    )))
}

// ログアウト
fn logout(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        format!("test")
    )))
}

// ユーザ登録
fn register(req: &mut Request) -> IronResult<Response> {
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
    let ref router = req.extensions.get::<Router>();
    let ref problem_id = router
        .unwrap()
        .find("id")
        .unwrap();
    
    Ok(Response::with((
        status::Ok,
        format!("test")
    )))
}    
              
fn user(req: &mut Request) -> IronResult<Response>{
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
        logout: post "/logout" => logout,
        regist: post "/regist" => register,
        user: get "/user" => user,
        problems: get "/problems" => problems,
        problem: get "/problem/:id" => problem,
        answer: post "/problem/:id" => answer,
    );

    let my_secret = b"verysecret".to_vec();
    let mut ch = Chain::new(router);
    ch.link_around(SessionStorage::new(SignedCookieBackend::new(my_secret)));
    let _res = Iron::new(ch).http("localhost:3000").unwrap();
    println!("on 3000");


    //PostgreSQL
//    let dsn = "postgres://dev:secret@localhost";
//       let conn = match Connection::connect(dsn, TlsMode::None) {
//           Ok(conn) => conn,
//           Err(e) => {
//              println!("Connection error: {}", e);
//               return;
//           }
//       };


//    create_hoge_table(&conn);
}
