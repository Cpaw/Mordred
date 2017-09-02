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
pub struct Question{
    pub id: i32,
    pub title: String,
    pub sentence: String,
    pub score: i16,
    pub accuracy: f64,
}


//データベース初期化(UserdataテーブルとQuestionテーブルの作成)
pub fn database_init(conn: &postgres::Connection){
    conn.batch_execute("
        CREATE TABLE userdata(
            username varchar not null unique,
            password varchar not null,
            cookie varchar,
            score int2 not null default 0
        );

        CREATE TABLE question(
          id serial primary key,
          title varchar,
          sentence text,
          score int2,
          accuracy float8
      );
      ").unwrap();
}


//ユーザーデータの登録
pub fn insert_userdata(conn: &postgres::Connection, username: String, password: String) {
    let rows_updated = conn.execute(
        "INSERT INTO userdata (username, password) VALUES ($1, $2)",
         &[&username, &password]).unwrap();
}

//問題の登録
pub fn insert_question(conn: &postgres::Connection, title: String, sentence: String, score: i16, accuracy: f64) {
    let rows_updated = conn.execute(
        "INSERT INTO question (title, sentence, score, accuracy) VALUES ($1, $2, $3, $4)",
         &[&title, &sentence, &score, &accuracy]).unwrap();
}

//問題の削除
pub fn delete_question(conn: &postgres::Connection, id: i32){
    let rows_updated = conn.execute(
        "DELETE FROM question WHERE id = $1",
         &[&id]).unwrap();
}


//ユーザー情報取り出し(比較部分で使う)
pub fn is_user_exists(conn: &postgres::Connection, user: String, pass: String)-> bool {
    //ユーザー名は重複しない
    for row in &conn.query("SELECT username FROM userdata WHERE username = $1 AND password = $2", &[&user, &pass]).unwrap() {
        return true;
    }
    false
}


//精度(accuracy)を返す
//pub fn get_accurace(conn: &postgres::Connection, id: i32, username: String)-> {
//}_

//Cookie登録
//pub fn set_cookie

//ユーザーにスコア加算(問題ID, ユーザー名)
pub fn add_score(conn: &postgres::Connection, id: i32, username: String){
    let rows_updated = conn.execute(
        "UPDATE userdata SET score = score + (SELECT score FROM question WHERE id = $1) WHERE username = $2",
         &[&id, &username]).unwrap();
}
