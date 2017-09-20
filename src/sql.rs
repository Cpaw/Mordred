extern crate postgres;
use postgres::{Connection, TlsMode};


//scoreはデフォルトで0が入る
pub struct Userdata{
    pub username: String,
    pub password: String,
    pub cookie: String,
    pub score: i16,
}

//idは自動採番
pub struct Promblem{
    pub id: i32,
    pub title: String,
    pub description: String,
    pub score: i16,
    pub accuracy: f64,
}


//データベース初期化(Userdataテーブルとproblemテーブルの作成)
pub fn database_init(conn: &postgres::Connection){
    conn.batch_execute("
        DROP TABLE IF EXISTS userdata CASCADE;
        DROP TABLE IF EXISTS problem CASCADE;

        CREATE TABLE userdata(
            username varchar not null unique,
            password varchar not null,
            cookie varchar,
            score int2 not null default 0
        );

        CREATE TABLE problem(
          id serial primary key,
          title varchar,
          description text,
          score int2,
          accuracy float8
      );
      ").unwrap();
}


//ユーザーデータの登録
pub fn insert_userdata(conn: &postgres::Connection, username: String, password: String) {
    conn.execute(
        "INSERT INTO userdata (username, password) VALUES ($1, $2)",
        &[&username, &password]).unwrap();
}

//問題の登録
pub fn insert_problem(conn: &postgres::Connection, title: String, description: String, score: i16, accuracy: f64) {
    conn.execute(
        "INSERT INTO problem (title, description, score, accuracy) VALUES ($1, $2, $3, $4)",
        &[&title, &description, &score, &accuracy]).unwrap();
}

//問題の削除
pub fn delete_problem(conn: &postgres::Connection, id: i32){
    conn.execute(
        "DELETE FROM problem WHERE id = $1",
        &[&id]).unwrap();
}

//問題一覧(id, title)
pub fn show_problems(conn: &postgres::Connection)->Vec<(i32, String)>{
/*
    let mut num:i64 = 0;

    //問題数取得
    for row in &conn.query("SELECT COUNT(*) FROM problem", &[]).unwrap() {
        let temp: i64 = row.get("count");
        num = temp;
    }
*/
    let mut pairs = Vec::new();

    for row in &conn.query("SELECT id, title FROM problem", &[]).unwrap() {
        let id: i32 = row.get("id");
        let title: String = row.get("title");
        pairs.push((id, title));
    }
    return pairs;
}


//ユーザー情報取り出し(比較部分で使う)
pub fn is_user_exists(conn: &postgres::Connection, user: String, pass: String)-> bool {
    //ユーザー名は重複しない
    for row in &conn.query("SELECT username FROM userdata WHERE username = $1 AND password = $2", &[&user, &pass]).unwrap() {
        return true;
    }
    false
}

//問題詳細を返す
pub fn get_description(conn: &postgres::Connection, id: i32)-> String {
    //ユーザー名は重複しない
    for row in &conn.query("SELECT description FROM problem WHERE id = $1", &[&id]).unwrap() {
        let description: String  = row.get("description");
        return description;
    }
    "".to_string()
}


//精度(accuracy)を返す
//pub fn get_accurace(conn: &postgres::Connection, id: i32, username: String)-> {
//}_

//Cookie登録
/*
pub fn set_cookie(conn: &postgres::Connection, id: i32, username: String){

}*/

//ユーザーにスコア加算(問題ID, ユーザー名)
pub fn add_score(conn: &postgres::Connection, id: i32, username: String){
    conn.execute(
        "UPDATE userdata SET score = score + (SELECT score FROM problem WHERE id = $1) WHERE username = $2",
        &[&id, &username]).unwrap();
}
