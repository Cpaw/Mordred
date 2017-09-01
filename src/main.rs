#[macro_use] extern crate iron;
#[macro_use] extern crate router;
extern crate rustc_serialize;
extern crate urlencoded;
extern crate iron_sessionstorage;

use iron::status;
use iron::modifiers::Redirect;

use iron_sessionstorage::traits::*;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

use urlencoded::UrlEncodedBody;

use iron::prelude::*;
//use router::Router;

mod hello_world;
mod set_greeting;

extern crate postgres;
use postgres::{Connection, TlsMode};

mod sql;
use sql::*;


struct Login {
    username: String
}

// ハッシュ管理
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
    Ok(Response::with((
        status::Ok,
        "text/html".parse::<iron::mime::Mime>().unwrap(),

        // ログインしているかどうか確認する処理を書く

    )))
}


// ユーザログイン                       
fn login_post(req: &mut Request) -> IronResult<Response> {
    let username = {
        let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
        iexpect!(formdata.get("username"))[0].to_owned()
    };

    try!(req.session().set(Login { username: username }));
    Ok(Response::with((status::Found, Redirect(url_for!(req, "greet")))))
}

// ログアウト
fn logout(req: &mut Request) -> IronResult<Response> {
    try!(req.session().clear());
    Ok(Response::with((status::Found, Redirect(url_for!(req, "greet")))))
}

// ユーザ登録
fn register(req: &mut Request) -> IronResult<Response> {
}

// 問題一覧
fn problems(req: &mut Request) -> IronResult<Response> {
}

// 問題詳細
fn problem(req: &mut Request) -> IronReslt<Response> {
    let ref router = req.extensions.get::<Router>();
    let ref problem_id = router
        .unwrap()
        .find("id")
        .unwrap();

    return Ok(Response::with(
        (status::Ok,
         format!("Hello {}", name).as_str())
}


fn main() {
    // ルーティング作成
    let router = router!(
        login: get "/login" => login,
        login_post: post "/login" => login_post,
        logout: post "/logout" => logout,
        regist: post "/regist" => regist,
        user: get "/user" => user,
        problems: get "/problems" => problems,
        problem: get "/problem/:id" => problem,
        
    );

    let my_secret = b"verysecret".to_vec();
    let mut ch = Chain::new(router);
    ch.link_around(SessionStorage::new(SignedCookieBackend::new(my_secret)));
    let _res = Iron::new(ch).http("localhost:3000").unwrap();
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


    create_hoge_table(&conn);
}
