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
    pub id: i16,
    pub title: String,
    pub sentence: String,
    pub score: i16,
    pub accuracy: i16,
}


//データベース初期化(UserdataテーブルとQuestionテーブルの作成)
pub fn database_init(conn: &postgres::Connection){
    //create table (userdata, question)
    conn.execute("CREATE TABLE userdata(
      username varchar not null unique,
      password varchar not null,
      cookie varchar,
      score int2 not null default 0
      )",
    &[]).unwrap();

    conn.execute("CREATE TABLE question(
      id serial primary key,
      title varchar,
      sentence text,
      score int2,
      accuracy int2
      )",
    &[]).unwrap();
}


//ユーザーデータの登録
pub fn insert_userdata(conn: &postgres::Connection, username: String, password: String) {
    let rows_updated = conn.execute(
        "INSERT INTO userdata (username, password) VALUES ($1, $2)",
         &[&username, &password]).unwrap();

    //println!("{} rows updated", rows_updated);
}

//問題の登録
pub fn insert_question(conn: &postgres::Connection, title: String, sentence: String, score: i16, accuracy: i16) {
    let rows_updated = conn.execute(
        "INSERT INTO question (title, sentence, score, accuracy) VALUES ($1, $2, $3, $4)",
         &[&title, &sentence, &score, &accuracy]).unwrap();

    //println!("{} rows updated", rows_updated);
}


//ユーザー情報取り出し(比較部分で使う)
pub fn is_user_exists(conn: &postgres::Connection, user: String)-> bool {
    //usernameはuniqueなのでfor文じゃなくてもいいかも
    for row in &conn.query("SELECT username FROM userdata WHERE username = $1", &[&user]).unwrap() {
        return true;
    }
    false
}
