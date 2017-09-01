extern crate postgres;
use postgres::{Connection, TlsMode};


pub struct Userdata{
    pub score: i16,
    pub username: String,
    pub password: String,
}

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
pub fn insert_userdata(conn: &postgres::Connection, user: String, pass: String) {
    let data = Userdata{
        username: user,
        password: pass,
        score: 0
    };

    let rows_updated = conn.execute(
        "INSERT INTO userdata (username, password) VALUES ($1, $2)",
         &[&data.username, &data.password]).unwrap();

    println!("{} rows updated", rows_updated);
}

//問題の登録
pub fn insert_question(conn: &postgres::Connection, tite: String, sent: String, scor: i16, accu: i16) {
    let data = Question {
        id: 0,
        title: tite,
        sentence: sent,
        score: scor,
        accuracy: accu
    };

    let rows_updated = conn.execute(
        "INSERT INTO question (title, sentence, score, accuracy) VALUES ($1, $2, $3, $4)",
         &[&data.title, &data.sentence, &data.score, &data.accuracy]).unwrap();

    println!("{} rows updated", rows_updated);
}


//ユーザー情報取り出し(比較部分で使う)
pub fn select_userdata(conn: &postgres::Connection, user: String)-> bool {
    //usernameはuniqueなのでfor文じゃなくてもいいかも
    for row in &conn.query("SELECT username FROM userdata WHERE username = $1", &[&user]).unwrap() {
        return true;
    }
    false
}




/*
   // Run
   stmt.execute(&[&me.counter]).ok().expect("Inserting counter failed");

   for row in &conn.query("SELECT id, hoge FROM hoge", &[]).unwrap() {
       let counter = Counter {
           id: row.get(0),
           counter: row.get(1)
       };
       println!("Found hoge {}", counter.counter);
   }*/
